// shared/interfaces/module_interface.rs
// IWS v1.0 - Module Interface
// Mendefinisikan trait Module untuk semua functional modules

use std::time::Duration;
use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;

use crate::shared::contracts::module_contract::{
    ModuleType, ModuleConfig, ModuleInput, ModuleOutput,
    ModuleStatus, ModuleCapabilities, ModuleDependency,
    ModuleRequirement, ModuleContractError, ValidationResult,
    CAP_NETWORK_SCAN, CAP_CONTENT_ANALYSIS, CAP_SECURITY_CHECK,
    CAP_INTELLIGENCE, CAP_INFRASTRUCTURE,
};

// ============================================================
// MODULE TRAIT
// ============================================================

#[async_trait]
pub trait Module: Send + Sync {
    type Error: std::error::Error + From<ModuleContractError> + Send + Sync;

    /// Eksekusi module
    async fn execute(&self, input: ModuleInput) -> Result<ModuleOutput, Self::Error>;

    /// Validasi konfigurasi
    fn validate_config(&self, config: &ModuleConfig) -> Result<ValidationResult, Self::Error>;

    /// Dapatkan capabilities
    fn capabilities(&self) -> ModuleCapabilities;

    /// Dapatkan versi
    fn version(&self) -> String {
        crate::shared::contracts::module_contract::API_VERSION.to_string()
    }

    /// Dapatkan nama
    fn name(&self) -> String;

    /// Dapatkan tipe
    fn module_type(&self) -> ModuleType;

    /// Dapatkan dependencies
    fn dependencies(&self) -> Vec<ModuleDependency>;

    /// Dapatkan requirements
    fn requirements(&self) -> Vec<ModuleRequirement>;

    /// Dapatkan schema konfigurasi
    fn config_schema(&self) -> serde_json::Value;

    /// Eksekusi dengan timeout
    async fn execute_with_timeout(
        &self,
        input: ModuleInput,
        timeout: Duration,
    ) -> Result<ModuleOutput, Self::Error> {
        tokio::select! {
            result = self.execute(input) => result,
            _ = tokio::time::sleep(timeout) => {
                Err(ModuleContractError::Timeout(
                    format!("Module {} timed out after {:?}", self.name(), timeout)
                ).into())
            }
        }
    }

    /// Eksekusi dengan retry
    async fn execute_with_retry(
        &self,
        input: ModuleInput,
        max_retries: u32,
        delay_ms: u64,
    ) -> Result<ModuleOutput, Self::Error> {
        let mut last_error = None;
        for attempt in 0..=max_retries {
            match self.execute(input.clone()).await {
                Ok(output) => return Ok(output),
                Err(e) if attempt < max_retries => {
                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                    last_error = Some(e);
                }
                Err(e) => return Err(e),
            }
        }
        Err(last_error.unwrap_or_else(|| {
            ModuleContractError::InternalError("Retry exhausted".to_string()).into()
        }))
    }

    /// Validasi input
    fn validate_input(&self, input: &ModuleInput) -> Result<(), Self::Error> {
        if input.target_url.is_empty() {
            return Err(ModuleContractError::InvalidInput(
                "target_url cannot be empty".to_string()
            ).into());
        }
        input.config.validate()?;
        Ok(())
    }

    /// Cek apakah module kompatibel dengan capability flag
    fn has_capability(&self, flag: u32) -> bool {
        self.capabilities().has_capability(flag)
    }

    /// Cek apakah module membutuhkan network
    fn requires_network(&self) -> bool {
        self.capabilities().requires_network
    }

    /// Cek apakah module siap dijalankan
    fn is_ready(&self) -> bool {
        true
    }

    /// Dapatkan estimasi durasi eksekusi
    fn estimated_duration(&self) -> Duration {
        Duration::from_millis(self.capabilities().average_duration_ms)
    }
}

// ============================================================
// MODULE CONTEXT
// ============================================================

#[derive(Debug, Clone)]
pub struct ModuleContext {
    pub module_id: Uuid,
    pub module_name: String,
    pub module_type: ModuleType,
    pub config: ModuleConfig,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: ModuleStatus,
    pub retry_count: u32,
    pub error_count: u32,
    pub metadata: serde_json::Value,
}

impl ModuleContext {
    pub fn new(module_name: &str, module_type: ModuleType, config: ModuleConfig) -> Self {
        ModuleContext {
            module_id: Uuid::new_v4(),
            module_name: module_name.to_string(),
            module_type,
            config,
            started_at: None,
            completed_at: None,
            status: ModuleStatus::Pending,
            retry_count: 0,
            error_count: 0,
            metadata: serde_json::json!({}),
        }
    }

    pub fn start(&mut self) {
        self.started_at = Some(chrono::Utc::now());
        self.status = ModuleStatus::Running;
    }

    pub fn complete(&mut self) {
        self.completed_at = Some(chrono::Utc::now());
        self.status = ModuleStatus::Completed;
    }

    pub fn fail(&mut self, _error: &str) {
        self.completed_at = Some(chrono::Utc::now());
        self.status = ModuleStatus::Failed;
        self.error_count += 1;
    }

    pub fn retry(&mut self) {
        self.retry_count += 1;
        self.status = ModuleStatus::Running;
    }

    pub fn duration_ms(&self) -> u64 {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => (end - start).num_milliseconds() as u64,
            (Some(start), None) => (chrono::Utc::now() - start).num_milliseconds() as u64,
            _ => 0,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.status == ModuleStatus::Completed
    }

    pub fn is_failed(&self) -> bool {
        self.status == ModuleStatus::Failed || self.status == ModuleStatus::TimedOut
    }

    pub fn can_retry(&self) -> bool {
        self.retry_count < self.config.retry_count
            && (self.status == ModuleStatus::Failed || self.status == ModuleStatus::TimedOut)
    }
}

// ============================================================
// MODULE PIPELINE
// ============================================================

pub struct ModulePipeline {
    modules: Vec<(String, Box<dyn Module<Error = ModuleContractError>>)>,
    execution_order: Vec<String>,
}

impl ModulePipeline {
    pub fn new() -> Self {
        ModulePipeline {
            modules: Vec::new(),
            execution_order: Vec::new(),
        }
    }

    pub fn add_module(
        &mut self,
        name: &str,
        module: Box<dyn Module<Error = ModuleContractError>>,
    ) -> Result<(), ModuleContractError> {
        if self.modules.iter().any(|(n, _)| n == name) {
            return Err(ModuleContractError::ModuleAlreadyRegistered(name.to_string()));
        }
        self.modules.push((name.to_string(), module));
        self.execution_order.push(name.to_string());
        Ok(())
    }

    pub fn remove_module(&mut self, name: &str) -> Result<(), ModuleContractError> {
        let pos = self.modules.iter().position(|(n, _)| n == name)
            .ok_or_else(|| ModuleContractError::ModuleNotFound(name.to_string()))?;
        self.modules.remove(pos);
        self.execution_order.retain(|n| n != name);
        Ok(())
    }

    pub fn get_module(&self, name: &str) -> Option<&dyn Module<Error = ModuleContractError>> {
        self.modules
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, m)| m.as_ref())
    }

    pub fn set_execution_order(&mut self, order: Vec<String>) -> Result<(), ModuleContractError> {
        let module_names: Vec<String> = self.modules.iter().map(|(n, _)| n.clone()).collect();
        for name in &order {
            if !module_names.contains(name) {
                return Err(ModuleContractError::ModuleNotFound(name.clone()));
            }
        }
        self.execution_order = order;
        Ok(())
    }

    pub async fn execute_all(
        &self,
        base_input: ModuleInput,
    ) -> Vec<(String, Result<ModuleOutput, ModuleContractError>)> {
        let mut results = Vec::new();

        for name in &self.execution_order {
            if let Some(module) = self.get_module(name) {
                let input = ModuleInput {
                    previous_results: results.iter()
                        .filter_map(|(_, r)| r.as_ref().ok().cloned())
                        .collect(),
                    ..base_input.clone()
                };

                let result = module.execute(input).await;
                results.push((name.clone(), result));
            }
        }

        results
    }

    pub async fn execute_parallel(
        &self,
        base_input: ModuleInput,
    ) -> Vec<(String, Result<ModuleOutput, ModuleContractError>)> {
        let mut handles = Vec::new();

        for name in &self.execution_order {
            if let Some(module) = self.get_module(name) {
                let input = base_input.clone();
                let name_clone = name.clone();

                // Karena Module bukan Clone, kita tidak bisa spawn parallel dengan &self
                // Dalam implementasi nyata, module akan di-wrap dalam Arc
                handles.push((name_clone, module.execute(input)));
            }
        }

        let mut results = Vec::new();
        for (name, handle) in handles {
            results.push((name, handle.await));
        }
        results
    }

    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    pub fn module_names(&self) -> Vec<String> {
        self.modules.iter().map(|(n, _)| n.clone()).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }
}

impl Default for ModulePipeline {
    fn default() -> Self {
        ModulePipeline::new()
    }
}

// ============================================================
// MODULE FACTORY
// ============================================================

pub trait ModuleFactory: Send + Sync {
    /// Buat module instance
    fn create(&self, config: ModuleConfig) -> Box<dyn Module<Error = ModuleContractError>>;

    /// Dapatkan tipe module yang dibuat
    fn module_type(&self) -> ModuleType;

    /// Dapatkan nama factory
    fn factory_name(&self) -> &str;

    /// Dapatkan default config
    fn default_config(&self) -> ModuleConfig;
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestModule {
        name: String,
        module_type: ModuleType,
        should_fail: bool,
    }

    impl TestModule {
        fn new(name: &str, module_type: ModuleType) -> Self {
            TestModule {
                name: name.to_string(),
                module_type,
                should_fail: false,
            }
        }

        fn failing(name: &str, module_type: ModuleType) -> Self {
            TestModule {
                name: name.to_string(),
                module_type,
                should_fail: true,
            }
        }
    }

    #[async_trait]
    impl Module for TestModule {
        type Error = ModuleContractError;

        async fn execute(&self, _input: ModuleInput) -> Result<ModuleOutput, Self::Error> {
            if self.should_fail {
                return Err(ModuleContractError::ModuleExecutionFailed(
                    format!("{} failed intentionally", self.name)
                ));
            }
            Ok(ModuleOutput::new(&self.name, self.module_type.clone()))
        }

        fn validate_config(&self, config: &ModuleConfig) -> Result<ValidationResult, Self::Error> {
            config.validate()?;
            Ok(ValidationResult::valid())
        }

        fn capabilities(&self) -> ModuleCapabilities {
            let mut caps = ModuleCapabilities::default();
            caps.set_capability(CAP_NETWORK_SCAN);
            caps
        }

        fn name(&self) -> String {
            self.name.clone()
        }

        fn module_type(&self) -> ModuleType {
            self.module_type.clone()
        }

        fn dependencies(&self) -> Vec<ModuleDependency> {
            vec![]
        }

        fn requirements(&self) -> Vec<ModuleRequirement> {
            vec![]
        }

        fn config_schema(&self) -> serde_json::Value {
            serde_json::json!({})
        }
    }

    fn make_input(url: &str) -> ModuleInput {
        let config = ModuleConfig::new("test", ModuleType::Network);
        ModuleInput::new(Uuid::new_v4(), url, config)
    }

    #[tokio::test]
    async fn test_module_execute_success() {
        let module = TestModule::new("dns_enum", ModuleType::Network);
        let input = make_input("https://example.com");
        let result = module.execute(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_module_execute_failure() {
        let module = TestModule::failing("bad_module", ModuleType::Security);
        let input = make_input("https://example.com");
        let result = module.execute(input).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_module_has_capability() {
        let module = TestModule::new("test", ModuleType::Network);
        assert!(module.has_capability(CAP_NETWORK_SCAN));
        assert!(!module.has_capability(CAP_SECURITY_CHECK));
    }

    #[test]
    fn test_module_context_lifecycle() {
        let mut ctx = ModuleContext::new(
            "test_module",
            ModuleType::Content,
            ModuleConfig::new("test", ModuleType::Content),
        );

        assert_eq!(ctx.status, ModuleStatus::Pending);
        assert_eq!(ctx.duration_ms(), 0);

        ctx.start();
        assert_eq!(ctx.status, ModuleStatus::Running);

        ctx.complete();
        assert_eq!(ctx.status, ModuleStatus::Completed);
        assert!(ctx.duration_ms() > 0);
    }

    #[test]
    fn test_module_context_fail_retry() {
        let mut ctx = ModuleContext::new(
            "flaky_module",
            ModuleType::Security,
            ModuleConfig::new("flaky", ModuleType::Security),
        );

        ctx.start();
        ctx.fail("timeout");
        assert_eq!(ctx.status, ModuleStatus::Failed);
        assert!(ctx.can_retry());

        ctx.retry();
        assert_eq!(ctx.status, ModuleStatus::Running);
        assert_eq!(ctx.retry_count, 1);
    }

    #[test]
    fn test_module_context_cannot_retry_max() {
        let mut config = ModuleConfig::new("limited", ModuleType::Intelligence);
        config.retry_count = 2;

        let mut ctx = ModuleContext::new("limited_module", ModuleType::Intelligence, config);
        ctx.start();
        ctx.fail("error");
        ctx.retry();
        ctx.fail("error again");
        ctx.retry();
        ctx.fail("error third time");

        assert_eq!(ctx.retry_count, 2);
        assert!(!ctx.can_retry());
    }

    #[tokio::test]
    async fn test_module_pipeline_add_execute() {
        let mut pipeline = ModulePipeline::new();
        pipeline.add_module("dns", Box::new(TestModule::new("dns", ModuleType::Network))).unwrap();
        pipeline.add_module("whois", Box::new(TestModule::new("whois", ModuleType::Intelligence))).unwrap();

        assert_eq!(pipeline.module_count(), 2);

        let input = make_input("https://example.com");
        let results = pipeline.execute_all(input).await;

        assert_eq!(results.len(), 2);
        assert!(results[0].1.is_ok());
        assert!(results[1].1.is_ok());
    }

    #[tokio::test]
    async fn test_module_pipeline_with_failure() {
        let mut pipeline = ModulePipeline::new();
        pipeline.add_module("good", Box::new(TestModule::new("good", ModuleType::Network))).unwrap();
        pipeline.add_module("bad", Box::new(TestModule::failing("bad", ModuleType::Security))).unwrap();

        let input = make_input("https://example.com");
        let results = pipeline.execute_all(input).await;

        assert_eq!(results.len(), 2);
        assert!(results[0].1.is_ok());
        assert!(results[1].1.is_err());
    }

    #[test]
    fn test_module_pipeline_duplicate() {
        let mut pipeline = ModulePipeline::new();
        pipeline.add_module("unique", Box::new(TestModule::new("unique", ModuleType::Network))).unwrap();
        let result = pipeline.add_module("unique", Box::new(TestModule::new("unique2", ModuleType::Network)));
        assert!(result.is_err());
    }

    #[test]
    fn test_module_pipeline_remove() {
        let mut pipeline = ModulePipeline::new();
        pipeline.add_module("removable", Box::new(TestModule::new("removable", ModuleType::Content))).unwrap();
        assert_eq!(pipeline.module_count(), 1);

        pipeline.remove_module("removable").unwrap();
        assert_eq!(pipeline.module_count(), 0);
        assert!(pipeline.is_empty());
    }
}
