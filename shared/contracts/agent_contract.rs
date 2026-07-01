// shared/contracts/agent_contract.rs
// IWS v1.0 - Agent Contract
// Mendefinisikan kontrak formal untuk semua agents dalam sistem

use std::time::Duration;
use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

// ============================================================
// AGENT STATE & TYPE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentState {
    Uninitialized,
    Initialized,
    Running,
    Paused,
    ShuttingDown,
    Shutdown,
    Error,
    Restarting,
}

impl fmt::Display for AgentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentState::Uninitialized => write!(f, "uninitialized"),
            AgentState::Initialized => write!(f, "initialized"),
            AgentState::Running => write!(f, "running"),
            AgentState::Paused => write!(f, "paused"),
            AgentState::ShuttingDown => write!(f, "shutting_down"),
            AgentState::Shutdown => write!(f, "shutdown"),
            AgentState::Error => write!(f, "error"),
            AgentState::Restarting => write!(f, "restarting"),
        }
    }
}

impl AgentState {
    pub fn is_active(&self) -> bool {
        matches!(self, AgentState::Running | AgentState::Restarting)
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, AgentState::Shutdown | AgentState::Error)
    }

    pub fn can_transition_to(&self, target: &AgentState) -> bool {
        use AgentState::*;
        match self {
            Uninitialized => matches!(target, Initialized | Error),
            Initialized => matches!(target, Running | Error | ShuttingDown),
            Running => matches!(target, Paused | Error | ShuttingDown),
            Paused => matches!(target, Running | Error | ShuttingDown),
            ShuttingDown => matches!(target, Shutdown | Error),
            Shutdown => false,
            Error => matches!(target, Restarting | ShuttingDown),
            Restarting => matches!(target, Initialized | Error),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentType {
    Reconnaissance,
    Analysis,
    Reporting,
    Monitoring,
    ModelIntegration,
    Custom(String),
}

impl fmt::Display for AgentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentType::Reconnaissance => write!(f, "reconnaissance"),
            AgentType::Analysis => write!(f, "analysis"),
            AgentType::Reporting => write!(f, "reporting"),
            AgentType::Monitoring => write!(f, "monitoring"),
            AgentType::ModelIntegration => write!(f, "model_integration"),
            AgentType::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

// ============================================================
// AGENT CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub agent_id: Uuid,
    pub agent_type: AgentType,
    pub agent_name: String,
    pub heartbeat_interval_secs: u64,
    pub heartbeat_timeout_secs: u64,
    pub max_restarts: u32,
    pub restart_delay_secs: u64,
    pub max_queue_size: usize,
    pub enable_persistence: bool,
    pub persistence_path: Option<String>,
    pub log_level: String,
    pub custom_config: serde_json::Value,
}

impl Default for AgentConfig {
    fn default() -> Self {
        AgentConfig {
            agent_id: Uuid::new_v4(),
            agent_type: AgentType::Custom("unknown".to_string()),
            agent_name: "unnamed_agent".to_string(),
            heartbeat_interval_secs: 30,
            heartbeat_timeout_secs: 90,
            max_restarts: 5,
            restart_delay_secs: 10,
            max_queue_size: 1000,
            enable_persistence: true,
            persistence_path: Some("./data/agent_states/".to_string()),
            log_level: "info".to_string(),
            custom_config: serde_json::json!({}),
        }
    }
}

impl AgentConfig {
    pub fn validate(&self) -> Result<(), AgentContractError> {
        if self.heartbeat_interval_secs < 5 || self.heartbeat_interval_secs > 300 {
            return Err(AgentContractError::InvalidConfiguration(
                format!("heartbeat_interval must be 5-300s, got {}", self.heartbeat_interval_secs)
            ));
        }
        if self.heartbeat_timeout_secs < self.heartbeat_interval_secs {
            return Err(AgentContractError::InvalidConfiguration(
                format!("heartbeat_timeout ({}) must be >= heartbeat_interval ({})",
                    self.heartbeat_timeout_secs, self.heartbeat_interval_secs)
            ));
        }
        if self.max_restarts > 100 {
            return Err(AgentContractError::InvalidConfiguration(
                format!("max_restarts must be 0-100, got {}", self.max_restarts)
            ));
        }
        if self.agent_name.is_empty() {
            return Err(AgentContractError::InvalidConfiguration(
                "agent_name cannot be empty".to_string()
            ));
        }
        Ok(())
    }
}

// ============================================================
// AGENT MESSAGE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub message_id: Uuid,
    pub sender_id: String,
    pub recipient_id: Option<String>,
    pub message_type: MessageType,
    pub payload: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub correlation_id: Option<Uuid>,
    pub priority: MessagePriority,
    pub ttl_secs: Option<u64>,
    pub metadata: HashMap<String, String>,
}

impl AgentMessage {
    pub fn new(
        sender_id: &str,
        message_type: MessageType,
        payload: Vec<u8>,
    ) -> Self {
        AgentMessage {
            message_id: Uuid::new_v4(),
            sender_id: sender_id.to_string(),
            recipient_id: None,
            message_type,
            payload,
            timestamp: Utc::now(),
            correlation_id: None,
            priority: MessagePriority::Normal,
            ttl_secs: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_recipient(mut self, recipient_id: &str) -> Self {
        self.recipient_id = Some(recipient_id.to_string());
        self
    }

    pub fn with_correlation(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_priority(mut self, priority: MessagePriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_ttl(mut self, ttl_secs: u64) -> Self {
        self.ttl_secs = Some(ttl_secs);
        self
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl_secs {
            let elapsed = Utc::now() - self.timestamp;
            elapsed.num_seconds() as u64 > ttl
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    Heartbeat,
    HeartbeatAck,
    StatusRequest,
    StatusResponse,
    TaskAssignment,
    TaskComplete,
    TaskFailed,
    ErrorReport,
    LogEntry,
    StateChange,
    EventNotification,
    Shutdown,
    Custom(String),
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::Heartbeat => write!(f, "heartbeat"),
            MessageType::HeartbeatAck => write!(f, "heartbeat_ack"),
            MessageType::StatusRequest => write!(f, "status_request"),
            MessageType::StatusResponse => write!(f, "status_response"),
            MessageType::TaskAssignment => write!(f, "task_assignment"),
            MessageType::TaskComplete => write!(f, "task_complete"),
            MessageType::TaskFailed => write!(f, "task_failed"),
            MessageType::ErrorReport => write!(f, "error_report"),
            MessageType::LogEntry => write!(f, "log_entry"),
            MessageType::StateChange => write!(f, "state_change"),
            MessageType::EventNotification => write!(f, "event_notification"),
            MessageType::Shutdown => write!(f, "shutdown"),
            MessageType::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    Low,
    Normal,
    High,
    Critical,
}

impl fmt::Display for MessagePriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessagePriority::Low => write!(f, "low"),
            MessagePriority::Normal => write!(f, "normal"),
            MessagePriority::High => write!(f, "high"),
            MessagePriority::Critical => write!(f, "critical"),
        }
    }
}

impl Default for MessagePriority {
    fn default() -> Self {
        MessagePriority::Normal
    }
}

// ============================================================
// AGENT EVENT
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvent {
    pub event_id: Uuid,
    pub agent_id: Uuid,
    pub event_type: AgentEventType,
    pub from_state: Option<AgentState>,
    pub to_state: Option<AgentState>,
    pub message: Option<String>,
    pub details: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

impl AgentEvent {
    pub fn new(
        agent_id: Uuid,
        event_type: AgentEventType,
        details: serde_json::Value,
    ) -> Self {
        AgentEvent {
            event_id: Uuid::new_v4(),
            agent_id,
            event_type,
            from_state: None,
            to_state: None,
            message: None,
            details,
            timestamp: Utc::now(),
        }
    }

    pub fn with_state_transition(
        mut self,
        from: AgentState,
        to: AgentState,
    ) -> Self {
        self.from_state = Some(from);
        self.to_state = Some(to);
        self
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentEventType {
    AgentStarted,
    AgentStopped,
    AgentPaused,
    AgentResumed,
    AgentCrashed,
    AgentRestarted,
    StateTransition,
    MessageSent,
    MessageReceived,
    TaskStarted,
    TaskCompleted,
    TaskFailed,
    HeartbeatSent,
    HeartbeatMissed,
    ErrorOccurred,
    WarningRaised,
    ResourceWarning,
    Custom(String),
}

impl fmt::Display for AgentEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentEventType::AgentStarted => write!(f, "agent_started"),
            AgentEventType::AgentStopped => write!(f, "agent_stopped"),
            AgentEventType::AgentPaused => write!(f, "agent_paused"),
            AgentEventType::AgentResumed => write!(f, "agent_resumed"),
            AgentEventType::AgentCrashed => write!(f, "agent_crashed"),
            AgentEventType::AgentRestarted => write!(f, "agent_restarted"),
            AgentEventType::StateTransition => write!(f, "state_transition"),
            AgentEventType::MessageSent => write!(f, "message_sent"),
            AgentEventType::MessageReceived => write!(f, "message_received"),
            AgentEventType::TaskStarted => write!(f, "task_started"),
            AgentEventType::TaskCompleted => write!(f, "task_completed"),
            AgentEventType::TaskFailed => write!(f, "task_failed"),
            AgentEventType::HeartbeatSent => write!(f, "heartbeat_sent"),
            AgentEventType::HeartbeatMissed => write!(f, "heartbeat_missed"),
            AgentEventType::ErrorOccurred => write!(f, "error_occurred"),
            AgentEventType::WarningRaised => write!(f, "warning_raised"),
            AgentEventType::ResourceWarning => write!(f, "resource_warning"),
            AgentEventType::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

// ============================================================
// AGENT STATUS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatus {
    pub agent_id: Uuid,
    pub agent_type: AgentType,
    pub agent_name: String,
    pub state: AgentState,
    pub uptime_secs: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub current_task: Option<String>,
    pub error_count: u64,
    pub restarts_count: u32,
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: f32,
    pub queue_size: usize,
    pub metadata: serde_json::Value,
}

impl AgentStatus {
    pub fn new(agent_id: Uuid, agent_type: AgentType, agent_name: &str) -> Self {
        AgentStatus {
            agent_id,
            agent_type,
            agent_name: agent_name.to_string(),
            state: AgentState::Uninitialized,
            uptime_secs: 0,
            messages_sent: 0,
            messages_received: 0,
            tasks_completed: 0,
            tasks_failed: 0,
            last_heartbeat: None,
            current_task: None,
            error_count: 0,
            restarts_count: 0,
            cpu_usage_percent: 0.0,
            memory_usage_mb: 0.0,
            queue_size: 0,
            metadata: serde_json::json!({}),
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.state.is_active()
            && self.last_heartbeat.is_some()
            && self.error_count < 10
    }
}

// ============================================================
// AGENT SUPERVISOR
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorConfig {
    pub check_interval_secs: u64,
    pub max_missed_heartbeats: u32,
    pub auto_restart: bool,
    pub max_restarts_per_hour: u32,
    pub cooldown_period_secs: u64,
    pub alert_on_restart: bool,
    pub alert_on_failure: bool,
    pub escalation_threshold: u32,
}

impl Default for SupervisorConfig {
    fn default() -> Self {
        SupervisorConfig {
            check_interval_secs: 10,
            max_missed_heartbeats: 3,
            auto_restart: true,
            max_restarts_per_hour: 10,
            cooldown_period_secs: 60,
            alert_on_restart: true,
            alert_on_failure: true,
            escalation_threshold: 5,
        }
    }
}

// ============================================================
// AGENT CONTRACT TRAIT
// ============================================================

#[async_trait]
pub trait AgentContract: Send + Sync {
    /// Inisialisasi agent
    /// Preconditions: state == Uninitialized
    /// Postconditions: state == Initialized
    async fn init(&mut self) -> Result<(), AgentContractError>;

    /// Menjalankan agent
    /// Preconditions: state == Initialized || state == Paused
    /// Postconditions: state == Running
    async fn run(&mut self) -> Result<(), AgentContractError>;

    /// Menjeda agent
    /// Preconditions: state == Running
    /// Postconditions: state == Paused
    async fn pause(&mut self) -> Result<(), AgentContractError>;

    /// Melanjutkan agent yang dijeda
    /// Preconditions: state == Paused
    /// Postconditions: state == Running
    async fn resume(&mut self) -> Result<(), AgentContractError>;

    /// Menghentikan agent
    /// Preconditions: state != Shutdown
    /// Postconditions: state == Shutdown
    async fn shutdown(&mut self) -> Result<(), AgentContractError>;

    /// Mendapatkan state agent saat ini
    fn get_state(&self) -> AgentState;

    /// Mendapatkan tipe agent
    fn get_type(&self) -> AgentType;

    /// Mendapatkan ID agent
    fn get_id(&self) -> Uuid;

    /// Mendapatkan nama agent
    fn get_name(&self) -> &str;

    /// Mengirim message ke agent lain
    async fn send_message(
        &self,
        msg: AgentMessage,
    ) -> Result<(), AgentContractError>;

    /// Menerima message dari agent lain
    async fn receive_message(
        &mut self,
    ) -> Result<Option<AgentMessage>, AgentContractError>;

    /// Mendapatkan status agent
    fn get_status(&self) -> AgentStatus;

    /// Mendaftarkan event callback
    fn on_event(&mut self, callback: Box<dyn Fn(AgentEvent) + Send + Sync>);

    /// Transisi state dengan validasi
    fn transition_state(
        &mut self,
        target: AgentState,
    ) -> Result<AgentState, AgentContractError> {
        let current = self.get_state();
        if !current.can_transition_to(&target) {
            return Err(AgentContractError::StateTransitionInvalid(
                format!("Cannot transition from {} to {}", current, target)
            ));
        }
        Ok(target)
    }

    /// Validasi konfigurasi agent
    fn validate_config(&self, config: &AgentConfig) -> Result<(), AgentContractError> {
        config.validate()
    }
}

// ============================================================
// AGENT MANAGER
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRegistry {
    pub agents: HashMap<Uuid, AgentStatus>,
    pub supervisor_config: SupervisorConfig,
    pub updated_at: DateTime<Utc>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        AgentRegistry {
            agents: HashMap::new(),
            supervisor_config: SupervisorConfig::default(),
            updated_at: Utc::now(),
        }
    }

    pub fn register(&mut self, status: AgentStatus) -> Result<(), AgentContractError> {
        if self.agents.contains_key(&status.agent_id) {
            return Err(AgentContractError::AlreadyRegistered(
                status.agent_id,
                status.agent_name.clone(),
            ));
        }
        self.agents.insert(status.agent_id, status);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn unregister(&mut self, agent_id: Uuid) -> Result<(), AgentContractError> {
        if !self.agents.contains_key(&agent_id) {
            return Err(AgentContractError::AgentNotFound(agent_id));
        }
        self.agents.remove(&agent_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn get(&self, agent_id: Uuid) -> Option<&AgentStatus> {
        self.agents.get(&agent_id)
    }

    pub fn list_by_type(&self, agent_type: AgentType) -> Vec<&AgentStatus> {
        self.agents
            .values()
            .filter(|a| a.agent_type == agent_type)
            .collect()
    }

    pub fn list_active(&self) -> Vec<&AgentStatus> {
        self.agents
            .values()
            .filter(|a| a.state.is_active())
            .collect()
    }

    pub fn count(&self) -> usize {
        self.agents.len()
    }

    pub fn count_by_state(&self, state: AgentState) -> usize {
        self.agents.values().filter(|a| a.state == state).count()
    }
}

// ============================================================
// ERROR TYPES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentContractError {
    InvalidConfiguration(String),
    AlreadyRunning(Uuid, String),
    AlreadyStopped(Uuid, String),
    AlreadyRegistered(Uuid, String),
    AgentNotFound(Uuid),
    StateTransitionInvalid(String),
    HeartbeatMissed(Uuid),
    MessageDeliveryFailed(String),
    MessageQueueFull(String),
    TaskExecutionFailed(String),
    AgentCrashed(Uuid, String),
    MaxRestartsExceeded(Uuid, u32),
    PersistenceFailed(String),
    Timeout(String),
    InternalError(String),
}

impl fmt::Display for AgentContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentContractError::InvalidConfiguration(msg) => write!(f, "Invalid config: {}", msg),
            AgentContractError::AlreadyRunning(id, name) => write!(f, "Agent {} ({}) already running", name, id),
            AgentContractError::AlreadyStopped(id, name) => write!(f, "Agent {} ({}) already stopped", name, id),
            AgentContractError::AlreadyRegistered(id, name) => write!(f, "Agent {} ({}) already registered", name, id),
            AgentContractError::AgentNotFound(id) => write!(f, "Agent not found: {}", id),
            AgentContractError::StateTransitionInvalid(msg) => write!(f, "Invalid transition: {}", msg),
            AgentContractError::HeartbeatMissed(id) => write!(f, "Heartbeat missed for agent: {}", id),
            AgentContractError::MessageDeliveryFailed(msg) => write!(f, "Message delivery failed: {}", msg),
            AgentContractError::MessageQueueFull(msg) => write!(f, "Message queue full: {}", msg),
            AgentContractError::TaskExecutionFailed(msg) => write!(f, "Task execution failed: {}", msg),
            AgentContractError::AgentCrashed(id, msg) => write!(f, "Agent {} crashed: {}", id, msg),
            AgentContractError::MaxRestartsExceeded(id, max) => write!(f, "Agent {} exceeded max restarts ({})", id, max),
            AgentContractError::PersistenceFailed(msg) => write!(f, "Persistence failed: {}", msg),
            AgentContractError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            AgentContractError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AgentContractError {}

impl AgentContractError {
    pub fn code(&self) -> &str {
        match self {
            AgentContractError::InvalidConfiguration(_) => "AG5001",
            AgentContractError::AlreadyRunning(_, _) => "AG5002",
            AgentContractError::AlreadyStopped(_, _) => "AG5003",
            AgentContractError::AlreadyRegistered(_, _) => "AG5004",
            AgentContractError::AgentNotFound(_) => "AG5005",
            AgentContractError::StateTransitionInvalid(_) => "AG5006",
            AgentContractError::HeartbeatMissed(_) => "AG5007",
            AgentContractError::MessageDeliveryFailed(_) => "AG5008",
            AgentContractError::MessageQueueFull(_) => "AG5009",
            AgentContractError::TaskExecutionFailed(_) => "AG5010",
            AgentContractError::AgentCrashed(_, _) => "AG5011",
            AgentContractError::MaxRestartsExceeded(_, _) => "AG5012",
            AgentContractError::PersistenceFailed(_) => "AG5013",
            AgentContractError::Timeout(_) => "AG5014",
            AgentContractError::InternalError(_) => "AG5015",
        }
    }

    pub fn severity(&self) -> &str {
        match self {
            AgentContractError::InvalidConfiguration(_) => "high",
            AgentContractError::AlreadyRunning(_, _) => "low",
            AgentContractError::AlreadyStopped(_, _) => "low",
            AgentContractError::AlreadyRegistered(_, _) => "low",
            AgentContractError::AgentNotFound(_) => "medium",
            AgentContractError::StateTransitionInvalid(_) => "high",
            AgentContractError::HeartbeatMissed(_) => "high",
            AgentContractError::MessageDeliveryFailed(_) => "medium",
            AgentContractError::MessageQueueFull(_) => "medium",
            AgentContractError::TaskExecutionFailed(_) => "high",
            AgentContractError::AgentCrashed(_, _) => "critical",
            AgentContractError::MaxRestartsExceeded(_, _) => "critical",
            AgentContractError::PersistenceFailed(_) => "high",
            AgentContractError::Timeout(_) => "medium",
            AgentContractError::InternalError(_) => "critical",
        }
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            AgentContractError::MessageDeliveryFailed(_)
                | AgentContractError::MessageQueueFull(_)
                | AgentContractError::Timeout(_)
                | AgentContractError::PersistenceFailed(_)
        )
    }
}

// ============================================================
// HEARTBEAT PAYLOAD
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatPayload {
    pub agent_id: Uuid,
    pub sequence: u64,
    pub timestamp: DateTime<Utc>,
    pub state: AgentState,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub queue_depth: usize,
    pub active_tasks: usize,
}

impl HeartbeatPayload {
    pub fn new(agent_id: Uuid, state: AgentState) -> Self {
        HeartbeatPayload {
            agent_id,
            sequence: 0,
            timestamp: Utc::now(),
            state,
            cpu_usage: 0.0,
            memory_usage: 0.0,
            queue_depth: 0,
            active_tasks: 0,
        }
    }

    pub fn with_sequence(mut self, sequence: u64) -> Self {
        self.sequence = sequence;
        self
    }

    pub fn with_resource_usage(mut self, cpu: f32, memory: f32) -> Self {
        self.cpu_usage = cpu;
        self.memory_usage = memory;
        self
    }
}

// ============================================================
// TASK ASSIGNMENT
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAssignment {
    pub task_id: Uuid,
    pub assigned_to: Uuid,
    pub assigned_by: String,
    pub task_type: TaskType,
    pub payload: serde_json::Value,
    pub priority: MessagePriority,
    pub deadline: Option<DateTime<Utc>>,
    pub retry_count: u32,
    pub max_retries: u32,
    pub created_at: DateTime<Utc>,
}

impl TaskAssignment {
    pub fn new(
        assigned_to: Uuid,
        assigned_by: &str,
        task_type: TaskType,
        payload: serde_json::Value,
    ) -> Self {
        TaskAssignment {
            task_id: Uuid::new_v4(),
            assigned_to,
            assigned_by: assigned_by.to_string(),
            task_type,
            payload,
            priority: MessagePriority::Normal,
            deadline: None,
            retry_count: 0,
            max_retries: 3,
            created_at: Utc::now(),
        }
    }

    pub fn with_deadline(mut self, deadline: DateTime<Utc>) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(deadline) = self.deadline {
            Utc::now() > deadline
        } else {
            false
        }
    }

    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskType {
    Reconnaissance,
    Scan,
    Analyze,
    Report,
    Monitor,
    Backup,
    Cleanup,
    Custom(String),
}

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskType::Reconnaissance => write!(f, "reconnaissance"),
            TaskType::Scan => write!(f, "scan"),
            TaskType::Analyze => write!(f, "analyze"),
            TaskType::Report => write!(f, "report"),
            TaskType::Monitor => write!(f, "monitor"),
            TaskType::Backup => write!(f, "backup"),
            TaskType::Cleanup => write!(f, "cleanup"),
            TaskType::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_state_transitions() {
        assert!(AgentState::Uninitialized.can_transition_to(&AgentState::Initialized));
        assert!(AgentState::Initialized.can_transition_to(&AgentState::Running));
        assert!(AgentState::Running.can_transition_to(&AgentState::Paused));
        assert!(AgentState::Paused.can_transition_to(&AgentState::Running));
        assert!(AgentState::Running.can_transition_to(&AgentState::ShuttingDown));
        assert!(AgentState::ShuttingDown.can_transition_to(&AgentState::Shutdown));
        assert!(AgentState::Error.can_transition_to(&AgentState::Restarting));
        assert!(AgentState::Restarting.can_transition_to(&AgentState::Initialized));
    }

    #[test]
    fn test_agent_state_invalid_transitions() {
        assert!(!AgentState::Shutdown.can_transition_to(&AgentState::Running));
        assert!(!AgentState::Uninitialized.can_transition_to(&AgentState::Running));
        assert!(!AgentState::Running.can_transition_to(&AgentState::Uninitialized));
    }

    #[test]
    fn test_agent_config_validation() {
        let mut config = AgentConfig::default();
        assert!(config.validate().is_ok());

        config.heartbeat_interval_secs = 1;
        assert!(config.validate().is_err());

        config.heartbeat_interval_secs = 30;
        config.heartbeat_timeout_secs = 10;
        assert!(config.validate().is_err());

        config.heartbeat_timeout_secs = 30;
        config.agent_name = "".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_agent_message_builder() {
        let msg = AgentMessage::new(
            "agent-001",
            MessageType::TaskAssignment,
            b"test payload".to_vec(),
        )
        .with_recipient("agent-002")
        .with_priority(MessagePriority::High)
        .with_ttl(60)
        .with_metadata("task_id", "task-001");

        assert_eq!(msg.sender_id, "agent-001");
        assert_eq!(msg.recipient_id, Some("agent-002".to_string()));
        assert_eq!(msg.priority, MessagePriority::High);
        assert_eq!(msg.ttl_secs, Some(60));
        assert_eq!(msg.metadata.get("task_id").unwrap(), "task-001");
    }

    #[test]
    fn test_agent_message_expiration() {
        let msg = AgentMessage::new(
            "agent-001",
            MessageType::Heartbeat,
            vec![],
        ).with_ttl(0);
        assert!(msg.is_expired());
    }

    #[test]
    fn test_agent_event_builder() {
        let event = AgentEvent::new(
            Uuid::new_v4(),
            AgentEventType::TaskCompleted,
            serde_json::json!({"result": "success"}),
        )
        .with_state_transition(AgentState::Running, AgentState::Running)
        .with_message("Task completed successfully");

        assert_eq!(event.event_type, AgentEventType::TaskCompleted);
        assert_eq!(event.from_state, Some(AgentState::Running));
        assert!(event.message.is_some());
    }

    #[test]
    fn test_agent_registry() {
        let mut registry = AgentRegistry::new();
        let status = AgentStatus::new(
            Uuid::new_v4(),
            AgentType::Reconnaissance,
            "recon-agent",
        );
        let agent_id = status.agent_id;

        assert!(registry.register(status).is_ok());
        assert_eq!(registry.count(), 1);

        assert!(registry.register(AgentStatus::new(
            agent_id,
            AgentType::Analysis,
            "dup-agent",
        )).is_err());

        let retrieved = registry.get(agent_id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().agent_name, "recon-agent");

        assert!(registry.unregister(agent_id).is_ok());
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_task_assignment() {
        let task = TaskAssignment::new(
            Uuid::new_v4(),
            "orchestrator",
            TaskType::Scan,
            serde_json::json!({"url": "https://example.com"}),
        );

        assert_eq!(task.task_type, TaskType::Scan);
        assert_eq!(task.retry_count, 0);
        assert!(task.can_retry());

        let overdue_deadline = Utc::now() - chrono::Duration::hours(1);
        let overdue_task = task.clone().with_deadline(overdue_deadline);
        assert!(overdue_task.is_overdue());
    }

    #[test]
    fn test_agent_error_codes() {
        let err = AgentContractError::HeartbeatMissed(Uuid::nil());
        assert_eq!(err.code(), "AG5007");

        let err = AgentContractError::MaxRestartsExceeded(Uuid::nil(), 5);
        assert_eq!(err.code(), "AG5012");
        assert!(err.to_string().contains("5"));
    }

    #[test]
    fn test_agent_status_health() {
        let mut status = AgentStatus::new(
            Uuid::new_v4(),
            AgentType::Monitoring,
            "monitor-agent",
        );
        assert!(!status.is_healthy());

        status.state = AgentState::Running;
        status.last_heartbeat = Some(Utc::now());
        assert!(status.is_healthy());
    }

    #[test]
    fn test_heartbeat_payload() {
        let payload = HeartbeatPayload::new(Uuid::new_v4(), AgentState::Running)
            .with_sequence(42)
            .with_resource_usage(45.5, 128.0);

        assert_eq!(payload.sequence, 42);
        assert_eq!(payload.cpu_usage, 45.5);
        assert_eq!(payload.memory_usage, 128.0);
        assert_eq!(payload.state, AgentState::Running);
    }

    #[test]
    fn test_agent_state_is_methods() {
        assert!(AgentState::Running.is_active());
        assert!(AgentState::Restarting.is_active());
        assert!(!AgentState::Paused.is_active());
        assert!(AgentState::Shutdown.is_terminal());
        assert!(AgentState::Error.is_terminal());
    }
}
