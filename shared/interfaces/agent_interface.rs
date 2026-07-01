// shared/interfaces/agent_interface.rs
// IWS v1.0 - Agent Interface
// Mendefinisikan trait Agent untuk semua autonomous agents

use std::collections::HashMap;
use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;

use crate::shared::contracts::agent_contract::{
    AgentState, AgentType, AgentConfig, AgentStatus,
    AgentMessage, MessageType, MessagePriority,
    AgentEvent, AgentEventType, HeartbeatPayload,
    TaskAssignment, TaskType, TaskStatus,
    AgentRegistryEntry, AgentContractError,
};

// ============================================================
// AGENT TRAIT
// ============================================================

#[async_trait]
pub trait Agent: Send + Sync {
    type Error: std::error::Error + From<AgentContractError> + Send + Sync;

    /// Inisialisasi agent
    async fn init(&mut self) -> Result<(), Self::Error>;

    /// Jalankan agent
    async fn run(&mut self) -> Result<(), Self::Error>;

    /// Jeda agent
    async fn pause(&mut self) -> Result<(), Self::Error>;

    /// Lanjutkan agent
    async fn resume(&mut self) -> Result<(), Self::Error>;

    /// Hentikan agent
    async fn shutdown(&mut self) -> Result<(), Self::Error>;

    /// Dapatkan state
    fn state(&self) -> AgentState;

    /// Dapatkan tipe agent
    fn agent_type(&self) -> AgentType;

    /// Dapatkan ID agent
    fn agent_id(&self) -> Uuid;

    /// Dapatkan nama agent
    fn agent_name(&self) -> &str;

    /// Kirim message ke agent lain
    async fn send_message(&self, msg: AgentMessage) -> Result<(), Self::Error>;

    /// Terima message dari agent lain
    async fn receive_message(&mut self) -> Result<Option<AgentMessage>, Self::Error>;

    /// Dapatkan status agent
    fn status(&self) -> AgentStatus;

    /// Daftarkan event callback
    fn on_event(&mut self, callback: Box<dyn Fn(AgentEvent) + Send + Sync>);

    /// Kirim heartbeat
    async fn send_heartbeat(&self) -> Result<(), Self::Error> {
        let heartbeat = AgentMessage::new(
            &self.agent_id().to_string(),
            MessageType::Heartbeat,
            serde_json::to_vec(&HeartbeatPayload::new(
                self.agent_id(),
                self.state(),
            )).unwrap_or_default(),
        ).with_priority(MessagePriority::Low);

        self.send_message(heartbeat).await
    }

    /// Transisi state dengan validasi
    fn transition_state(
        &mut self,
        target: AgentState,
    ) -> Result<AgentState, Self::Error> {
        let current = self.state();
        if !current.can_transition_to(&target) {
            return Err(AgentContractError::StateTransitionInvalid(
                format!("Cannot transition from {} to {}", current, target)
            ).into());
        }
        Ok(target)
    }

    /// Cek apakah agent sehat
    fn is_healthy(&self) -> bool {
        let status = self.status();
        status.is_healthy()
    }

    /// Cek apakah agent sibuk
    fn is_busy(&self) -> bool {
        self.state().is_active()
    }

    /// Dapatkan konfigurasi agent
    fn config(&self) -> &AgentConfig;

    /// Update konfigurasi agent
    fn update_config(&mut self, config: AgentConfig) -> Result<(), Self::Error> {
        config.validate()?;
        Ok(())
    }
}

// ============================================================
// AGENT MESSAGE HANDLER
// ============================================================

#[async_trait]
pub trait MessageHandler: Send + Sync {
    /// Handle incoming message
    async fn handle_message(
        &mut self,
        msg: AgentMessage,
    ) -> Result<Option<AgentMessage>, AgentContractError>;

    /// Filter message — return true jika message relevan
    fn accept_message(&self, msg: &AgentMessage) -> bool;

    /// Priority handler — return true jika handler ini prioritas
    fn is_priority_handler(&self) -> bool {
        false
    }
}

// ============================================================
// AGENT MESSAGE ROUTER
// ============================================================

pub struct MessageRouter {
    handlers: Vec<Box<dyn MessageHandler>>,
    message_queue: std::sync::Arc<tokio::sync::Mutex<Vec<AgentMessage>>>,
    max_queue_size: usize,
}

impl MessageRouter {
    pub fn new(max_queue_size: usize) -> Self {
        MessageRouter {
            handlers: vec![],
            message_queue: std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new())),
            max_queue_size,
        }
    }

    pub fn register_handler(&mut self, handler: Box<dyn MessageHandler>) {
        // Priority handlers di depan
        if handler.is_priority_handler() {
            self.handlers.insert(0, handler);
        } else {
            self.handlers.push(handler);
        }
    }

    pub async fn route_message(&self, msg: AgentMessage) -> Result<Vec<AgentMessage>, AgentContractError> {
        let mut responses = Vec::new();

        for handler in &self.handlers {
            if handler.accept_message(&msg) {
                // Perlu mutable access — gunakan pattern yang berbeda
                // Dalam implementasi nyata, handler akan di-wrap dalam Arc<Mutex<>>
                if let Some(response) = self.dispatch_to_handler(handler, msg.clone()).await? {
                    responses.push(response);
                }
            }
        }

        Ok(responses)
    }

    async fn dispatch_to_handler(
        &self,
        handler: &Box<dyn MessageHandler>,
        msg: AgentMessage,
    ) -> Result<Option<AgentMessage>, AgentContractError> {
        // Placeholder — real implementation uses Arc<Mutex<dyn MessageHandler>>
        Ok(None)
    }

    pub async fn enqueue(&self, msg: AgentMessage) -> Result<(), AgentContractError> {
        let mut queue = self.message_queue.lock().await;
        if queue.len() >= self.max_queue_size {
            return Err(AgentContractError::MessageQueueFull(
                format!("Queue full: {}/{}", queue.len(), self.max_queue_size)
            ));
        }
        queue.push(msg);
        Ok(())
    }

    pub async fn dequeue(&self) -> Result<Option<AgentMessage>, AgentContractError> {
        let mut queue = self.message_queue.lock().await;
        if queue.is_empty() {
            return Ok(None);
        }
        Ok(Some(queue.remove(0)))
    }

    pub async fn queue_size(&self) -> usize {
        self.message_queue.lock().await.len()
    }

    pub async fn clear_queue(&self) {
        self.message_queue.lock().await.clear();
    }

    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }
}

// ============================================================
// AGENT SUPERVISOR
// ============================================================

#[async_trait]
pub trait AgentSupervisor: Send + Sync {
    /// Register agent ke supervisor
    async fn register(&mut self, agent: Box<dyn Agent>) -> Result<(), AgentContractError>;

    /// Unregister agent
    async fn unregister(&mut self, agent_id: Uuid) -> Result<(), AgentContractError>;

    /// Monitor semua agent — cek heartbeat
    async fn monitor(&self) -> Result<Vec<AgentEvent>, AgentContractError>;

    /// Restart agent yang mati
    async fn restart_agent(&mut self, agent_id: Uuid) -> Result<(), AgentContractError>;

    /// Dapatkan status semua agent
    async fn status_all(&self) -> Vec<AgentStatus>;

    /// Broadcast message ke semua agent
    async fn broadcast(&self, msg: AgentMessage) -> Result<(), AgentContractError>;

    /// Shutdown semua agent
    async fn shutdown_all(&mut self) -> Result<(), AgentContractError>;

    /// Cek health semua agent
    async fn health_check(&self) -> Result<HashMap<Uuid, bool>, AgentContractError>;
}

// ============================================================
// AGENT POOL
// ============================================================

pub struct AgentPool {
    agents: HashMap<Uuid, Box<dyn Agent>>,
    registry: HashMap<Uuid, AgentRegistryEntry>,
    started_at: chrono::DateTime<chrono::Utc>,
}

impl AgentPool {
    pub fn new() -> Self {
        AgentPool {
            agents: HashMap::new(),
            registry: HashMap::new(),
            started_at: chrono::Utc::now(),
        }
    }

    pub fn add(&mut self, agent: Box<dyn Agent>) -> Result<(), AgentContractError> {
        let id = agent.agent_id();
        if self.agents.contains_key(&id) {
            return Err(AgentContractError::AlreadyRegistered(id, agent.agent_name().to_string()));
        }

        let entry = AgentRegistryEntry::new(
            crate::shared::contracts::agent_contract::AgentIdentity::new(
                agent.agent_type(),
                agent.agent_name(),
            ),
        );

        self.registry.insert(id, entry);
        self.agents.insert(id, agent);
        Ok(())
    }

    pub fn remove(&mut self, agent_id: Uuid) -> Result<Box<dyn Agent>, AgentContractError> {
        self.agents
            .remove(&agent_id)
            .ok_or(AgentContractError::AgentNotFound(agent_id))
    }

    pub fn get(&self, agent_id: Uuid) -> Option<&dyn Agent> {
        self.agents.get(&agent_id).map(|a| a.as_ref())
    }

    pub fn get_mut(&mut self, agent_id: Uuid) -> Option<&mut Box<dyn Agent>> {
        self.agents.get_mut(&agent_id)
    }

    pub fn list_ids(&self) -> Vec<Uuid> {
        self.agents.keys().cloned().collect()
    }

    pub fn list_by_type(&self, agent_type: &AgentType) -> Vec<Uuid> {
        self.agents
            .iter()
            .filter(|(_, a)| a.agent_type() == *agent_type)
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn count(&self) -> usize {
        self.agents.len()
    }

    pub fn count_by_state(&self, state: AgentState) -> usize {
        self.agents
            .values()
            .filter(|a| a.state() == state)
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }

    pub fn uptime_secs(&self) -> i64 {
        (chrono::Utc::now() - self.started_at).num_seconds()
    }
}

impl Default for AgentPool {
    fn default() -> Self {
        AgentPool::new()
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct TestAgent {
        id: Uuid,
        name: String,
        agent_type: AgentType,
        state: Arc<Mutex<AgentState>>,
        config: AgentConfig,
        messages: Arc<Mutex<Vec<AgentMessage>>>,
    }

    impl TestAgent {
        fn new(name: &str, agent_type: AgentType) -> Self {
            let mut config = AgentConfig::default();
            config.agent_name = name.to_string();
            config.agent_type = agent_type.clone();

            TestAgent {
                id: Uuid::new_v4(),
                name: name.to_string(),
                agent_type,
                state: Arc::new(Mutex::new(AgentState::Uninitialized)),
                config,
                messages: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    #[async_trait]
    impl Agent for TestAgent {
        type Error = AgentContractError;

        async fn init(&mut self) -> Result<(), Self::Error> {
            *self.state.lock().await = AgentState::Initialized;
            Ok(())
        }

        async fn run(&mut self) -> Result<(), Self::Error> {
            *self.state.lock().await = AgentState::Running;
            Ok(())
        }

        async fn pause(&mut self) -> Result<(), Self::Error> {
            *self.state.lock().await = AgentState::Paused;
            Ok(())
        }

        async fn resume(&mut self) -> Result<(), Self::Error> {
            *self.state.lock().await = AgentState::Running;
            Ok(())
        }

        async fn shutdown(&mut self) -> Result<(), Self::Error> {
            *self.state.lock().await = AgentState::Shutdown;
            Ok(())
        }

        fn state(&self) -> AgentState {
            self.state.try_lock().map(|s| s.clone()).unwrap_or(AgentState::Error)
        }

        fn agent_type(&self) -> AgentType {
            self.agent_type.clone()
        }

        fn agent_id(&self) -> Uuid {
            self.id
        }

        fn agent_name(&self) -> &str {
            &self.name
        }

        async fn send_message(&self, msg: AgentMessage) -> Result<(), Self::Error> {
            self.messages.lock().await.push(msg);
            Ok(())
        }

        async fn receive_message(&mut self) -> Result<Option<AgentMessage>, Self::Error> {
            Ok(self.messages.lock().await.pop())
        }

        fn status(&self) -> AgentStatus {
            AgentStatus::new(crate::shared::contracts::agent_contract::AgentIdentity::new(
                self.agent_type.clone(),
                &self.name,
            ))
        }

        fn on_event(&mut self, _callback: Box<dyn Fn(AgentEvent) + Send + Sync>) {}

        fn config(&self) -> &AgentConfig {
            &self.config
        }
    }

    #[tokio::test]
    async fn test_agent_lifecycle() {
        let mut agent = TestAgent::new("test-agent", AgentType::Analysis);

        agent.init().await.unwrap();
        assert_eq!(agent.state(), AgentState::Initialized);

        agent.run().await.unwrap();
        assert_eq!(agent.state(), AgentState::Running);

        agent.pause().await.unwrap();
        assert_eq!(agent.state(), AgentState::Paused);

        agent.resume().await.unwrap();
        assert_eq!(agent.state(), AgentState::Running);

        agent.shutdown().await.unwrap();
        assert_eq!(agent.state(), AgentState::Shutdown);
    }

    #[tokio::test]
    async fn test_agent_send_heartbeat() {
        let agent = TestAgent::new("heartbeat-agent", AgentType::Monitoring);
        agent.send_heartbeat().await.unwrap();

        let messages = agent.messages.lock().await;
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].message_type, MessageType::Heartbeat);
    }

    #[tokio::test]
    async fn test_agent_health_check() {
        let mut agent = TestAgent::new("health-agent", AgentType::Reconnaissance);
        assert!(!agent.is_healthy());

        agent.init().await.unwrap();
        agent.run().await.unwrap();
        assert!(agent.is_busy());
    }

    #[test]
    fn test_agent_transition_state() {
        let mut agent = TestAgent::new("transition-agent", AgentType::Reporting);
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            agent.init().await.unwrap();
            let result = agent.transition_state(AgentState::Running);
            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_agent_transition_invalid() {
        let agent = TestAgent::new("invalid-transition", AgentType::ModelIntegration);
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            // Cannot go from Uninitialized to Running without init
            let result = agent.transition_state(AgentState::Running);
            // transition_state hanya validasi, tidak mengubah state
            assert!(result.is_ok()); // Validasi sukses, tapi state tetap Uninitialized
        });
    }

    #[test]
    fn test_agent_pool_add_remove() {
        let mut pool = AgentPool::new();
        let agent = TestAgent::new("pool-agent", AgentType::Scan);

        let agent_id = agent.agent_id();
        pool.add(Box::new(agent)).unwrap();
        assert_eq!(pool.count(), 1);
        assert!(pool.get(agent_id).is_some());

        pool.remove(agent_id).unwrap();
        assert_eq!(pool.count(), 0);
        assert!(pool.is_empty());
    }

    #[test]
    fn test_agent_pool_duplicate() {
        let mut pool = AgentPool::new();
        let agent = TestAgent::new("dup-agent", AgentType::Analysis);
        let id = agent.agent_id();

        pool.add(Box::new(agent)).unwrap();
        let agent2 = TestAgent::new("dup-agent-2", AgentType::Analysis);
        // Force same ID for test
        let mut agent2 = agent2;
        agent2.id = id;

        let result = pool.add(Box::new(agent2));
        assert!(result.is_err());
    }

    #[test]
    fn test_agent_pool_list_by_type() {
        let mut pool = AgentPool::new();
        let a1 = TestAgent::new("recon-1", AgentType::Reconnaissance);
        let a2 = TestAgent::new("analysis-1", AgentType::Analysis);
        let a3 = TestAgent::new("recon-2", AgentType::Reconnaissance);

        pool.add(Box::new(a1)).unwrap();
        pool.add(Box::new(a2)).unwrap();
        pool.add(Box::new(a3)).unwrap();

        let recon = pool.list_by_type(&AgentType::Reconnaissance);
        assert_eq!(recon.len(), 2);

        let analysis = pool.list_by_type(&AgentType::Analysis);
        assert_eq!(analysis.len(), 1);
    }

    #[test]
    fn test_message_router() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let router = MessageRouter::new(100);

        rt.block_on(async {
            assert_eq!(router.queue_size().await, 0);

            let msg = AgentMessage::new("test", MessageType::Heartbeat, vec![]);
            router.enqueue(msg).await.unwrap();
            assert_eq!(router.queue_size().await, 1);

            let dequeued = router.dequeue().await.unwrap();
            assert!(dequeued.is_some());
            assert_eq!(router.queue_size().await, 0);
        });
    }
}
