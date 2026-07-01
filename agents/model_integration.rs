// agents/model_integration.rs
// IWS v1.0 - Model Integration Agent
// Menghubungkan core system dengan AI models dan mengelola inference

use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use lru::LruCache;
use std::num::NonZeroUsize;

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub version: String,
    pub model_type: String,
    pub loaded: bool,
    pub inference_count: u64,
    pub average_time_ms: f64,
}

#[derive(Debug, Clone)]
pub struct InferenceRequest {
    pub model_name: String,
    pub input_data: Vec<u8>,
    pub request_id: Uuid,
    pub priority: u8,
}

#[derive(Debug, Clone)]
pub struct InferenceResult {
    pub request_id: Uuid,
    pub model_name: String,
    pub output_data: Vec<u8>,
    pub confidence: f64,
    pub inference_time_ms: u64,
    pub cached: bool,
}

pub struct ModelIntegrationAgent {
    models: RwLock<HashMap<String, ModelInfo>>,
    cache: RwLock<LruCache<String, InferenceResult>>,
    inference_count: RwLock<u64>,
    total_time_ms: RwLock<f64>,
}

impl ModelIntegrationAgent {
    pub fn new(cache_size: usize) -> Self {
        ModelIntegrationAgent {
            models: RwLock::new(HashMap::new()),
            cache: RwLock::new(LruCache::new(NonZeroUsize::new(cache_size.max(1)).unwrap())),
            inference_count: RwLock::new(0),
            total_time_ms: RwLock::new(0.0),
        }
    }

    pub async fn register_model(&self, name: &str, version: &str, model_type: &str) {
        let mut models = self.models.write().await;
        models.insert(name.to_string(), ModelInfo {
            name: name.to_string(),
            version: version.to_string(),
            model_type: model_type.to_string(),
            loaded: false,
            inference_count: 0,
            average_time_ms: 0.0,
        });
    }

    pub async fn load_model(&self, name: &str) -> Result<(), String> {
        let mut models = self.models.write().await;
        if let Some(model) = models.get_mut(name) {
            model.loaded = true;
            Ok(())
        } else {
            Err(format!("Model '{}' not registered", name))
        }
    }

    pub async fn run_inference(&self, request: &InferenceRequest) -> Result<InferenceResult, String> {
        // Check cache
        let cache_key = format!("{}:{}", request.model_name, self.hash_data(&request.input_data));
        {
            let mut cache = self.cache.write().await;
            if let Some(cached) = cache.get(&cache_key) {
                let mut result = cached.clone();
                result.cached = true;
                return Ok(result);
            }
        }

        // Check model exists
        let models = self.models.read().await;
        let model = models.get(&request.model_name)
            .ok_or_else(|| format!("Model '{}' not found", request.model_name))?;

        if !model.loaded {
            return Err(format!("Model '{}' not loaded", request.model_name));
        }
        drop(models);

        let start = std::time::Instant::now();

        // Production: run actual model inference via Python bridge or ONNX runtime
        let output = self.execute_model(&request.model_name, &request.input_data).await?;

        let elapsed = start.elapsed();
        let elapsed_ms = elapsed.as_millis() as u64;

        // Update stats
        {
            let mut count = self.inference_count.write().await;
            *count += 1;
            let mut total = self.total_time_ms.write().await;
            *total += elapsed_ms as f64;
        }

        // Update model stats
        {
            let mut models = self.models.write().await;
            if let Some(model) = models.get_mut(&request.model_name) {
                model.inference_count += 1;
                let total = model.average_time_ms * (model.inference_count - 1) as f64;
                model.average_time_ms = (total + elapsed_ms as f64) / model.inference_count as f64;
            }
        }

        let result = InferenceResult {
            request_id: request.request_id,
            model_name: request.model_name.clone(),
            output_data: output,
            confidence: 0.95,
            inference_time_ms: elapsed_ms,
            cached: false,
        };

        // Store in cache
        {
            let mut cache = self.cache.write().await;
            cache.put(cache_key, result.clone());
        }

        Ok(result)
    }

    async fn execute_model(&self, model_name: &str, input: &[u8]) -> Result<Vec<u8>, String> {
        // Production: bridge to Python/TensorFlow/ONNX
        // Placeholder: return echo
        let response = format!(r#"{{"model":"{}","output":"processed","input_size":{}}}"#, model_name, input.len());
        Ok(response.into_bytes())
    }

    fn hash_data(&self, data: &[u8]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish().to_string()
    }

    pub async fn unload_model(&self, name: &str) -> Result<(), String> {
        let mut models = self.models.write().await;
        if let Some(model) = models.get_mut(name) {
            model.loaded = false;
            Ok(())
        } else {
            Err(format!("Model '{}' not found", name))
        }
    }

    pub async fn get_models(&self) -> Vec<ModelInfo> {
        self.models.read().await.values().cloned().collect()
    }

    pub async fn get_stats(&self) -> ModelStats {
        let count = *self.inference_count.read().await;
        let total = *self.total_time_ms.read().await;
        let avg = if count > 0 { total / count as f64 } else { 0.0 };
        let cache_size = self.cache.read().await.len();

        ModelStats {
            total_inferences: count,
            average_time_ms: avg,
            cache_size,
            models_loaded: self.models.read().await.values().filter(|m| m.loaded).count(),
        }
    }

    pub async fn clear_cache(&self) {
        self.cache.write().await.clear();
    }

    pub async fn batch_inference(&self, requests: &[InferenceRequest]) -> Vec<Result<InferenceResult, String>> {
        let mut results = Vec::with_capacity(requests.len());
        for request in requests {
            results.push(self.run_inference(request).await);
        }
        results
    }
}

#[derive(Debug, Clone)]
pub struct ModelStats {
    pub total_inferences: u64,
    pub average_time_ms: f64,
    pub cache_size: usize,
    pub models_loaded: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_and_load_model() {
        let agent = ModelIntegrationAgent::new(100);
        agent.register_model("test_model", "1.0", "classifier").await;
        assert!(agent.load_model("test_model").await.is_ok());
        let models = agent.get_models().await;
        assert_eq!(models.len(), 1);
        assert!(models[0].loaded);
    }

    #[tokio::test]
    async fn test_run_inference() {
        let agent = ModelIntegrationAgent::new(100);
        agent.register_model("nlp", "1.0", "nlp").await;
        agent.load_model("nlp").await.unwrap();

        let request = InferenceRequest {
            model_name: "nlp".into(),
            input_data: b"test input".to_vec(),
            request_id: Uuid::new_v4(),
            priority: 1,
        };

        let result = agent.run_inference(&request).await.unwrap();
        assert_eq!(result.model_name, "nlp");
        assert!(result.confidence > 0.0);
        assert!(!result.cached);
    }

    #[tokio::test]
    async fn test_inference_caching() {
        let agent = ModelIntegrationAgent::new(100);
        agent.register_model("test", "1.0", "test").await;
        agent.load_model("test").await.unwrap();

        let request = InferenceRequest {
            model_name: "test".into(),
            input_data: b"same input".to_vec(),
            request_id: Uuid::new_v4(),
            priority: 1,
        };

        let r1 = agent.run_inference(&request).await.unwrap();
        assert!(!r1.cached);

        let r2 = agent.run_inference(&request).await.unwrap();
        assert!(r2.cached);
    }

    #[tokio::test]
    async fn test_unload_model() {
        let agent = ModelIntegrationAgent::new(100);
        agent.register_model("temp", "1.0", "test").await;
        agent.load_model("temp").await.unwrap();
        assert!(agent.unload_model("temp").await.is_ok());

        let models = agent.get_models().await;
        assert!(!models[0].loaded);
    }

    #[tokio::test]
    async fn test_batch_inference() {
        let agent = ModelIntegrationAgent::new(100);
        agent.register_model("batch", "1.0", "test").await;
        agent.load_model("batch").await.unwrap();

        let requests: Vec<InferenceRequest> = (0..5).map(|i| InferenceRequest {
            model_name: "batch".into(),
            input_data: vec![i as u8; 10],
            request_id: Uuid::new_v4(),
            priority: 1,
        }).collect();

        let results = agent.batch_inference(&requests).await;
        assert_eq!(results.len(), 5);
        assert!(results.iter().all(|r| r.is_ok()));
    }
}
