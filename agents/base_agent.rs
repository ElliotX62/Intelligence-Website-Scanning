// agents/base_agent.rs
// IWS v1.0 - Base Agent
// Menyediakan base class/template untuk semua agents

use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentState {
    Uninitialized, Initialized, Running, Paused, ShuttingDown, Shutdown, Error, Restarting,
}

impl AgentState {
    pub fn can_transition_to(&self, target: &AgentState) -> bool {
        use AgentState::*;
        matches!((self, target),
            (Uninitialized, Initialized) | (Initialized, Running) | (Running, Paused) |
            (Paused, Running) | (Running, ShuttingDown) | (ShuttingDown, Shutdown) |
            (Error, Restarting) | (Restarting, Initialized) |
            (_, Error) | (_, ShuttingDown)
        )
    }
}

#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub agent_id: Uuid,
    pub agent_type: String,
    pub agent_name: String,
    pub heartbeat_interval_secs: u64,
    pub heartbeat_timeout_secs: u64,
    pub max_restarts: u32,
    pub restart_delay_secs: u64,
}

impl Default for AgentConfig {
    fn default() -> Self {
        AgentConfig {
            agent_id: Uuid::new_v4(), agent_type: "base".into(), agent_name: "unnamed".into(),
            heartbeat_interval_secs: 30, heartbeat_timeout_secs: 90,
            max_restarts: 5, restart_delay_secs: 10,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AgentMessage {
    pub message_id: Uuid,
    pub sender_id: String,
    pub recipient_id: Option<String>,
    pub message_type: String,
    pub payload: Vec<u8>,
    pub timestamp: DateTime<Utc>,
}

impl AgentMessage {
    pub fn new(sender: &str, msg_type: &str, payload: Vec<u8>) -> Self {
        AgentMessage {
            message_id: Uuid::new_v4(), sender_id: sender.to_string(),
            recipient_id: None, message_type: msg_type.to_string(),
            payload, timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AgentStatus {
    pub agent_id: Uuid,
    pub state: AgentState,
    pub uptime_secs: u64,
    pub messages_processed: u64,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub last_heartbeat: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait Agent: Send + Sync {
    async fn init(&mut self) -> Result<(), String>;
    async fn run(&mut self) -> Result<(), String>;
    async fn pause(&mut self) -> Result<(), String>;
    async fn resume(&mut self) -> Result<(), String>;
    async fn shutdown(&mut self) -> Result<(), String>;
    fn get_state(&self) -> AgentState;
    fn get_id(&self) -> Uuid;
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &str;
    async fn send_message(&self, msg: AgentMessage) -> Result<(), String>;
    async fn receive_message(&mut self) -> Option<AgentMessage>;
    fn get_status(&self) -> AgentStatus;
}

pub struct MessageBus {
    queues: RwLock<HashMap<String, Vec<AgentMessage>>>,
}

impl MessageBus {
    pub fn new() -> Self {
        MessageBus { queues: RwLock::new(HashMap::new()) }
    }

    pub async fn send(&self, recipient: &str, msg: AgentMessage) -> Result<(), String> {
        let mut queues = self.queues.write().await;
        queues.entry(recipient.to_string()).or_insert_with(Vec::new).push(msg);
        Ok(())
    }

    pub async fn receive(&self, recipient: &str) -> Option<AgentMessage> {
        let mut queues = self.queues.write().await;
        queues.get_mut(recipient).and_then(|q| if q.is_empty() { None } else { Some(q.remove(0)) })
    }

    pub async fn broadcast(&self, sender: &str, msg_type: &str, payload: Vec<u8>, recipients: &[String]) {
        for recipient in recipients {
            let msg = AgentMessage::new(sender, msg_type, payload.clone());
            let _ = self.send(recipient, msg).await;
        }
    }
}

pub struct AgentRegistry {
    agents: RwLock<HashMap<Uuid, Box<dyn Agent>>>,
    bus: Arc<MessageBus>,
}

impl AgentRegistry {
    pub fn new(bus: Arc<MessageBus>) -> Self {
        AgentRegistry { agents: RwLock::new(HashMap::new()), bus }
    }

    pub async fn register(&self, agent: Box<dyn Agent>) -> Result<(), String> {
        let id = agent.get_id();
        let mut agents = self.agents.write().await;
        if agents.contains_key(&id) {
            return Err(format!("Agent {} already registered", id));
        }
        agents.insert(id, agent);
        Ok(())
    }

    pub async fn unregister(&self, id: &Uuid) -> Option<Box<dyn Agent>> {
        self.agents.write().await.remove(id)
    }

    pub async fn get(&self, id: &Uuid) -> Option<Box<dyn Agent>> {
        self.agents.read().await.get(id).map(|_| unimplemented!("Cannot return owned from ref"))
    }

    pub async fn broadcast(&self, sender: &str, msg_type: &str, payload: Vec<u8>) {
        let agents = self.agents.read().await;
        let ids: Vec<String> = agents.keys().map(|id| id.to_string()).collect();
        drop(agents);
        self.bus.broadcast(sender, msg_type, payload, &ids).await;
    }

    pub async fn count(&self) -> usize {
        self.agents.read().await.len()
    }

    pub async fn list_ids(&self) -> Vec<Uuid> {
        self.agents.read().await.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestAgent {
        id: Uuid, name: String, state: AgentState, messages: Vec<AgentMessage>,
    }

    impl TestAgent {
        fn new(name: &str) -> Self {
            TestAgent { id: Uuid::new_v4(), name: name.into(), state: AgentState::Uninitialized, messages: vec![] }
        }
    }

    #[async_trait]
    impl Agent for TestAgent {
        async fn init(&mut self) -> Result<(), String> { self.state = AgentState::Initialized; Ok(()) }
        async fn run(&mut self) -> Result<(), String> { self.state = AgentState::Running; Ok(()) }
        async fn pause(&mut self) -> Result<(), String> { self.state = AgentState::Paused; Ok(()) }
        async fn resume(&mut self) -> Result<(), String> { self.state = AgentState::Running; Ok(()) }
        async fn shutdown(&mut self) -> Result<(), String> { self.state = AgentState::Shutdown; Ok(()) }
        fn get_state(&self) -> AgentState { self.state.clone() }
        fn get_id(&self) -> Uuid { self.id }
        fn get_name(&self) -> &str { &self.name }
        fn get_type(&self) -> &str { "test" }
        async fn send_message(&self, _msg: AgentMessage) -> Result<(), String> { Ok(()) }
        async fn receive_message(&mut self) -> Option<AgentMessage> { self.messages.pop() }
        fn get_status(&self) -> AgentStatus {
            AgentStatus { agent_id: self.id, state: self.state.clone(), uptime_secs: 0, messages_processed: 0, tasks_completed: 0, tasks_failed: 0, last_heartbeat: None }
        }
    }

    #[tokio::test]
    async fn test_agent_lifecycle() {
        let mut agent = TestAgent::new("test");
        agent.init().await.unwrap();
        assert_eq!(agent.get_state(), AgentState::Initialized);
        agent.run().await.unwrap();
        assert_eq!(agent.get_state(), AgentState::Running);
        agent.pause().await.unwrap();
        assert_eq!(agent.get_state(), AgentState::Paused);
        agent.resume().await.unwrap();
        assert_eq!(agent.get_state(), AgentState::Running);
        agent.shutdown().await.unwrap();
        assert_eq!(agent.get_state(), AgentState::Shutdown);
    }

    #[tokio::test]
    async fn test_message_bus() {
        let bus = MessageBus::new();
        let msg = AgentMessage::new("agent1", "test", vec![1, 2, 3]);
        bus.send("agent2", msg).await.unwrap();
        let received = bus.receive("agent2").await;
        assert!(received.is_some());
    }

    #[tokio::test]
    async fn test_agent_registry() {
        let bus = Arc::new(MessageBus::new());
        let registry = AgentRegistry::new(bus);
        let agent = TestAgent::new("reg-test");
        let id = agent.get_id();
        registry.register(Box::new(agent)).await.unwrap();
        assert_eq!(registry.count().await, 1);
        registry.unregister(&id).await;
        assert_eq!(registry.count().await, 0);
    }
}
