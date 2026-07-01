// shared/types/agent_types.rs
// IWS v1.0 - Agent Types
// Mendefinisikan tipe data untuk agent system

use std::fmt;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::common_types::{Severity, Priority};

// ============================================================
// AGENT IDENTITY
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

impl AgentState {
    pub fn is_active(&self) -> bool {
        matches!(self, AgentState::Running | AgentState::Restarting)
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, AgentState::Shutdown | AgentState::Error)
    }

    pub fn is_operational(&self) -> bool {
        matches!(self, AgentState::Initialized | AgentState::Running | AgentState::Paused)
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    pub agent_id: Uuid,
    pub agent_type: AgentType,
    pub agent_name: String,
    pub version: String,
    pub hostname: String,
    pub pid: u32,
    pub started_at: DateTime<Utc>,
    pub labels: HashMap<String, String>,
}

impl AgentIdentity {
    pub fn new(agent_type: AgentType, name: &str) -> Self {
        AgentIdentity {
            agent_id: Uuid::new_v4(),
            agent_type,
            agent_name: name.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            hostname: hostname::get()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            pid: std::process::id(),
            started_at: Utc::now(),
            labels: HashMap::new(),
        }
    }

    pub fn with_label(mut self, key: &str, value: &str) -> Self {
        self.labels.insert(key.to_string(), value.to_string());
        self
    }

    pub fn short_id(&self) -> String {
        self.agent_id.to_string().chars().take(8).collect()
    }
}

// ============================================================
// AGENT STATUS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatus {
    pub identity: AgentIdentity,
    pub state: AgentState,
    pub uptime_secs: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub tasks_in_progress: usize,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub current_task: Option<String>,
    pub error_count: u64,
    pub restarts_count: u32,
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: f32,
    pub queue_size: usize,
    pub last_error: Option<String>,
    pub metadata: serde_json::Value,
}

impl AgentStatus {
    pub fn new(identity: AgentIdentity) -> Self {
        AgentStatus {
            identity,
            state: AgentState::Uninitialized,
            uptime_secs: 0,
            messages_sent: 0,
            messages_received: 0,
            tasks_completed: 0,
            tasks_failed: 0,
            tasks_in_progress: 0,
            last_heartbeat: None,
            current_task: None,
            error_count: 0,
            restarts_count: 0,
            cpu_usage_percent: 0.0,
            memory_usage_mb: 0.0,
            queue_size: 0,
            last_error: None,
            metadata: serde_json::json!({}),
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.state.is_active()
            && self.last_heartbeat.is_some()
            && self.error_count < 10
    }

    pub fn is_overloaded(&self) -> bool {
        self.cpu_usage_percent > 90.0 || self.memory_usage_mb > 1024.0 || self.queue_size > 1000
    }

    pub fn success_rate(&self) -> f32 {
        let total = self.tasks_completed + self.tasks_failed;
        if total == 0 {
            return 100.0;
        }
        (self.tasks_completed as f32 / total as f32) * 100.0
    }

    pub fn heartbeat_age_secs(&self) -> Option<u64> {
        self.last_heartbeat.map(|hb| {
            (Utc::now() - hb).num_seconds() as u64
        })
    }
}

// ============================================================
// WORKFLOW REQUEST & STATUS (H1 FIX)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRequest {
    pub workflow_id: Uuid,
    pub target_url: String,
    pub profile: String,
    pub options: WorkflowOptions,
    pub tags: Vec<String>,
    pub priority: WorkflowPriority,
    pub callback_url: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
}

impl WorkflowRequest {
    pub fn new(target_url: &str, profile: &str, created_by: &str) -> Self {
        WorkflowRequest {
            workflow_id: Uuid::new_v4(),
            target_url: target_url.to_string(),
            profile: profile.to_string(),
            options: WorkflowOptions::default(),
            tags: vec![],
            priority: WorkflowPriority::Normal,
            callback_url: None,
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
            created_by: created_by.to_string(),
        }
    }

    pub fn with_options(mut self, options: WorkflowOptions) -> Self {
        self.options = options;
        self
    }

    pub fn with_priority(mut self, priority: WorkflowPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowOptions {
    pub timeout_secs: u64,
    pub max_pages: usize,
    pub follow_redirects: bool,
    pub respect_robots: bool,
    pub enable_js: bool,
    pub scan_depth: u8,
    pub thread_count: usize,
    pub delay_ms: u64,
    pub retry_count: u32,
    pub modules: Vec<String>,
    pub exclude_modules: Vec<String>,
    pub output_formats: Vec<String>,
    pub notify_on_complete: bool,
    pub notify_on_error: bool,
}

impl Default for WorkflowOptions {
    fn default() -> Self {
        WorkflowOptions {
            timeout_secs: 1800,
            max_pages: 500,
            follow_redirects: true,
            respect_robots: true,
            enable_js: false,
            scan_depth: 3,
            thread_count: 50,
            delay_ms: 100,
            retry_count: 3,
            modules: vec![],
            exclude_modules: vec![],
            output_formats: vec!["json".to_string(), "html".to_string()],
            notify_on_complete: false,
            notify_on_error: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkflowPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl fmt::Display for WorkflowPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkflowPriority::Low => write!(f, "low"),
            WorkflowPriority::Normal => write!(f, "normal"),
            WorkflowPriority::High => write!(f, "high"),
            WorkflowPriority::Critical => write!(f, "critical"),
        }
    }
}

impl Default for WorkflowPriority {
    fn default() -> Self {
        WorkflowPriority::Normal
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStatus {
    pub workflow_id: Uuid,
    pub state: WorkflowState,
    pub progress: f32,
    pub current_step: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub elapsed_secs: u64,
    pub estimated_remaining_secs: u64,
    pub error: Option<String>,
    pub error_count: usize,
    pub warning_count: usize,
    pub modules_done: Vec<String>,
    pub modules_pending: Vec<String>,
    pub modules_failed: Vec<String>,
    pub pages_done: usize,
    pub pages_total: usize,
    pub findings_count: usize,
    pub metadata: serde_json::Value,
    pub updated_at: DateTime<Utc>,
}

impl WorkflowStatus {
    pub fn new(workflow_id: Uuid) -> Self {
        let now = Utc::now();
        WorkflowStatus {
            workflow_id,
            state: WorkflowState::Idle,
            progress: 0.0,
            current_step: "initializing".to_string(),
            start_time: now,
            end_time: None,
            elapsed_secs: 0,
            estimated_remaining_secs: 0,
            error: None,
            error_count: 0,
            warning_count: 0,
            modules_done: vec![],
            modules_pending: vec![],
            modules_failed: vec![],
            pages_done: 0,
            pages_total: 0,
            findings_count: 0,
            metadata: serde_json::json!({}),
            updated_at: now,
        }
    }

    pub fn update_progress(&mut self, progress: f32, current_step: &str) {
        self.progress = progress.min(100.0).max(0.0);
        self.current_step = current_step.to_string();
        self.updated_at = Utc::now();
        self.elapsed_secs = (Utc::now() - self.start_time).num_seconds() as u64;

        if progress > 0.0 && self.elapsed_secs > 0 {
            let total_estimated = self.elapsed_secs as f32 / (progress / 100.0);
            self.estimated_remaining_secs = (total_estimated - self.elapsed_secs as f32) as u64;
        }
    }

    pub fn is_complete(&self) -> bool {
        matches!(self.state, WorkflowState::Complete)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self.state, WorkflowState::Failed | WorkflowState::Cancelled)
    }

    pub fn is_running(&self) -> bool {
        matches!(self.state, WorkflowState::Preparing | WorkflowState::Scanning | WorkflowState::Analyzing | WorkflowState::Reporting)
    }

    pub fn module_completion_percent(&self) -> f32 {
        let total = self.modules_done.len() + self.modules_pending.len() + self.modules_failed.len();
        if total == 0 { return 0.0; }
        (self.modules_done.len() as f32 / total as f32) * 100.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkflowState {
    Idle,
    Preparing,
    Scanning,
    Analyzing,
    Reporting,
    Complete,
    Failed,
    Cancelled,
    Paused,
}

impl fmt::Display for WorkflowState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkflowState::Idle => write!(f, "idle"),
            WorkflowState::Preparing => write!(f, "preparing"),
            WorkflowState::Scanning => write!(f, "scanning"),
            WorkflowState::Analyzing => write!(f, "analyzing"),
            WorkflowState::Reporting => write!(f, "reporting"),
            WorkflowState::Complete => write!(f, "complete"),
            WorkflowState::Failed => write!(f, "failed"),
            WorkflowState::Cancelled => write!(f, "cancelled"),
            WorkflowState::Paused => write!(f, "paused"),
        }
    }
}

// ============================================================
// AGENT MESSAGE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    Heartbeat,
    HeartbeatAck,
    StatusRequest,
    StatusResponse,
    TaskAssignment,
    TaskComplete,
    TaskFailed,
    TaskProgress,
    ErrorReport,
    LogEntry,
    StateChange,
    EventNotification,
    Shutdown,
    ShutdownAck,
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
            MessageType::TaskProgress => write!(f, "task_progress"),
            MessageType::ErrorReport => write!(f, "error_report"),
            MessageType::LogEntry => write!(f, "log_entry"),
            MessageType::StateChange => write!(f, "state_change"),
            MessageType::EventNotification => write!(f, "event_notification"),
            MessageType::Shutdown => write!(f, "shutdown"),
            MessageType::ShutdownAck => write!(f, "shutdown_ack"),
            MessageType::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub message_id: Uuid,
    pub sender_id: String,
    pub recipient_id: Option<String>,
    pub message_type: MessageType,
    pub payload: Vec<u8>,
    pub content_type: String,
    pub timestamp: DateTime<Utc>,
    pub correlation_id: Option<Uuid>,
    pub priority: MessagePriority,
    pub ttl_secs: Option<u64>,
    pub retry_count: u32,
    pub max_retries: u32,
    pub metadata: HashMap<String, String>,
}

impl AgentMessage {
    pub fn new(sender_id: &str, message_type: MessageType, payload: Vec<u8>) -> Self {
        AgentMessage {
            message_id: Uuid::new_v4(),
            sender_id: sender_id.to_string(),
            recipient_id: None,
            message_type,
            payload,
            content_type: "application/octet-stream".to_string(),
            timestamp: Utc::now(),
            correlation_id: None,
            priority: MessagePriority::Normal,
            ttl_secs: None,
            retry_count: 0,
            max_retries: 3,
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

    pub fn with_json_payload<T: Serialize>(mut self, data: &T) -> Result<Self, serde_json::Error> {
        self.payload = serde_json::to_vec(data)?;
        self.content_type = "application/json".to_string();
        Ok(self)
    }

    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl_secs {
            (Utc::now() - self.timestamp).num_seconds() as u64 > ttl
        } else {
            false
        }
    }

    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }

    pub fn size_bytes(&self) -> usize {
        self.payload.len()
    }
}

// ============================================================
// AGENT EVENT
// ============================================================

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
    TaskProgress,
    HeartbeatSent,
    HeartbeatMissed,
    ErrorOccurred,
    WarningRaised,
    ResourceThresholdExceeded,
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
            AgentEventType::TaskProgress => write!(f, "task_progress"),
            AgentEventType::HeartbeatSent => write!(f, "heartbeat_sent"),
            AgentEventType::HeartbeatMissed => write!(f, "heartbeat_missed"),
            AgentEventType::ErrorOccurred => write!(f, "error_occurred"),
            AgentEventType::WarningRaised => write!(f, "warning_raised"),
            AgentEventType::ResourceThresholdExceeded => write!(f, "resource_threshold_exceeded"),
            AgentEventType::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvent {
    pub event_id: Uuid,
    pub agent_id: Uuid,
    pub event_type: AgentEventType,
    pub from_state: Option<AgentState>,
    pub to_state: Option<AgentState>,
    pub message: Option<String>,
    pub details: serde_json::Value,
    pub severity: Severity,
    pub timestamp: DateTime<Utc>,
}

impl AgentEvent {
    pub fn new(agent_id: Uuid, event_type: AgentEventType, details: serde_json::Value) -> Self {
        AgentEvent {
            event_id: Uuid::new_v4(),
            agent_id,
            event_type,
            from_state: None,
            to_state: None,
            message: None,
            details,
            severity: Severity::Info,
            timestamp: Utc::now(),
        }
    }

    pub fn with_state_transition(mut self, from: AgentState, to: AgentState) -> Self {
        self.from_state = Some(from);
        self.to_state = Some(to);
        self.event_type = AgentEventType::StateTransition;
        self
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }
}

// ============================================================
// TASK ASSIGNMENT
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskType {
    Reconnaissance,
    Scan,
    Analyze,
    Report,
    Monitor,
    Backup,
    Cleanup,
    HealthCheck,
    ModelTrain,
    ModelEvaluate,
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
            TaskType::HealthCheck => write!(f, "health_check"),
            TaskType::ModelTrain => write!(f, "model_train"),
            TaskType::ModelEvaluate => write!(f, "model_evaluate"),
            TaskType::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Assigned,
    InProgress,
    Completed,
    Failed,
    Cancelled,
    Retrying,
    TimedOut,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "pending"),
            TaskStatus::Assigned => write!(f, "assigned"),
            TaskStatus::InProgress => write!(f, "in_progress"),
            TaskStatus::Completed => write!(f, "completed"),
            TaskStatus::Failed => write!(f, "failed"),
            TaskStatus::Cancelled => write!(f, "cancelled"),
            TaskStatus::Retrying => write!(f, "retrying"),
            TaskStatus::TimedOut => write!(f, "timed_out"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAssignment {
    pub task_id: Uuid,
    pub assigned_to: Uuid,
    pub assigned_by: String,
    pub task_type: TaskType,
    pub payload: serde_json::Value,
    pub priority: Priority,
    pub status: TaskStatus,
    pub deadline: Option<DateTime<Utc>>,
    pub retry_count: u32,
    pub max_retries: u32,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub result: Option<serde_json::Value>,
    pub metadata: HashMap<String, String>,
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
            priority: Priority::P3,
            status: TaskStatus::Pending,
            deadline: None,
            retry_count: 0,
            max_retries: 3,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            error_message: None,
            result: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_deadline(mut self, deadline: DateTime<Utc>) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
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
            && matches!(self.status, TaskStatus::Failed | TaskStatus::TimedOut)
    }

    pub fn duration_secs(&self) -> Option<u64> {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => Some((end - start).num_seconds() as u64),
            (Some(start), None) => Some((Utc::now() - start).num_seconds() as u64),
            _ => None,
        }
    }

    pub fn mark_started(&mut self) {
        self.status = TaskStatus::InProgress;
        self.started_at = Some(Utc::now());
    }

    pub fn mark_completed(&mut self, result: serde_json::Value) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.result = Some(result);
    }

    pub fn mark_failed(&mut self, error: &str) {
        self.status = TaskStatus::Failed;
        self.completed_at = Some(Utc::now());
        self.error_message = Some(error.to_string());
    }
}

// ============================================================
// HEARTBEAT
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    pub agent_id: Uuid,
    pub sequence: u64,
    pub state: AgentState,
    pub timestamp: DateTime<Utc>,
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: f32,
    pub queue_depth: usize,
    pub active_tasks: usize,
    pub uptime_secs: u64,
}

impl Heartbeat {
    pub fn new(agent_id: Uuid, state: AgentState) -> Self {
        Heartbeat {
            agent_id,
            sequence: 0,
            state,
            timestamp: Utc::now(),
            cpu_usage_percent: 0.0,
            memory_usage_mb: 0.0,
            queue_depth: 0,
            active_tasks: 0,
            uptime_secs: 0,
        }
    }

    pub fn age_secs(&self) -> u64 {
        (Utc::now() - self.timestamp).num_seconds() as u64
    }

    pub fn is_stale(&self, timeout_secs: u64) -> bool {
        self.age_secs() > timeout_secs
    }
}

// ============================================================
// AGENT REGISTRY ENTRY
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRegistryEntry {
    pub identity: AgentIdentity,
    pub status: AgentStatus,
    pub registered_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub enabled: bool,
    pub capabilities: Vec<String>,
    pub dependencies: Vec<String>,
}

impl AgentRegistryEntry {
    pub fn new(identity: AgentIdentity) -> Self {
        let now = Utc::now();
        AgentRegistryEntry {
            status: AgentStatus::new(identity.clone()),
            identity,
            registered_at: now,
            last_updated: now,
            enabled: true,
            capabilities: vec![],
            dependencies: vec![],
        }
    }

    pub fn is_stale(&self, max_age_secs: u64) -> bool {
        (Utc::now() - self.last_updated).num_seconds() as u64 > max_age_secs
    }

    pub fn needs_restart(&self) -> bool {
        self.status.state == AgentState::Error && self.enabled
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_identity_short_id() {
        let identity = AgentIdentity::new(AgentType::Analysis, "test-agent");
        assert_eq!(identity.short_id().len(), 8);
    }

    #[test]
    fn test_agent_state_is_active() {
        assert!(AgentState::Running.is_active());
        assert!(AgentState::Restarting.is_active());
        assert!(!AgentState::Paused.is_active());
        assert!(!AgentState::Shutdown.is_active());
    }

    #[test]
    fn test_agent_status_healthy() {
        let identity = AgentIdentity::new(AgentType::Monitoring, "monitor-1");
        let mut status = AgentStatus::new(identity);
        assert!(!status.is_healthy());

        status.state = AgentState::Running;
        status.last_heartbeat = Some(Utc::now());
        assert!(status.is_healthy());
    }

    #[test]
    fn test_agent_status_overloaded() {
        let identity = AgentIdentity::new(AgentType::Analysis, "analyzer-1");
        let mut status = AgentStatus::new(identity);
        status.cpu_usage_percent = 95.0;
        assert!(status.is_overloaded());

        status.cpu_usage_percent = 50.0;
        status.memory_usage_mb = 2048.0;
        assert!(status.is_overloaded());
    }

    #[test]
    fn test_agent_status_success_rate() {
        let identity = AgentIdentity::new(AgentType::Reconnaissance, "recon-1");
        let mut status = AgentStatus::new(identity);
        assert_eq!(status.success_rate(), 100.0);

        status.tasks_completed = 8;
        status.tasks_failed = 2;
        assert!((status.success_rate() - 80.0).abs() < 0.1);
    }

    #[test]
    fn test_workflow_request_builder() {
        let options = WorkflowOptions::default();
        let request = WorkflowRequest::new("https://example.com", "moderate", "admin")
            .with_options(options)
            .with_priority(WorkflowPriority::High);

        assert_eq!(request.target_url, "https://example.com");
        assert_eq!(request.profile, "moderate");
        assert_eq!(request.priority, WorkflowPriority::High);
    }

    #[test]
    fn test_workflow_status_progress() {
        let mut status = WorkflowStatus::new(Uuid::new_v4());
        status.update_progress(50.0, "Scanning ports");
        assert_eq!(status.progress, 50.0);
        assert_eq!(status.current_step, "Scanning ports");
    }

    #[test]
    fn test_workflow_state_display() {
        assert_eq!(WorkflowState::Idle.to_string(), "idle");
        assert_eq!(WorkflowState::Complete.to_string(), "complete");
    }

    #[test]
    fn test_workflow_status_is_methods() {
        let mut status = WorkflowStatus::new(Uuid::new_v4());
        assert!(!status.is_complete());
        assert!(!status.is_failed());
        assert!(!status.is_running());

        status.state = WorkflowState::Scanning;
        assert!(status.is_running());

        status.state = WorkflowState::Complete;
        assert!(status.is_complete());
    }

    #[test]
    fn test_workflow_status_module_completion() {
        let mut status = WorkflowStatus::new(Uuid::new_v4());
        status.modules_done = vec!["dns".to_string(), "whois".to_string()];
        status.modules_pending = vec!["port".to_string(), "ssl".to_string()];
        status.modules_failed = vec!["xss".to_string()];

        let pct = status.module_completion_percent();
        assert!((pct - 40.0).abs() < 0.1);
    }

    #[test]
    fn test_agent_message_builder() {
        let msg = AgentMessage::new("agent-1", MessageType::TaskAssignment, vec![1, 2, 3])
            .with_recipient("agent-2")
            .with_priority(MessagePriority::High)
            .with_ttl(30)
            .with_metadata("trace_id", "abc123");

        assert_eq!(msg.sender_id, "agent-1");
        assert_eq!(msg.recipient_id, Some("agent-2".to_string()));
        assert_eq!(msg.priority, MessagePriority::High);
        assert!(msg.can_retry());
    }

    #[test]
    fn test_agent_message_expired() {
        let msg = AgentMessage::new("agent-1", MessageType::Heartbeat, vec![])
            .with_ttl(0);
        assert!(msg.is_expired());
    }

    #[test]
    fn test_agent_event_builder() {
        let event = AgentEvent::new(
            Uuid::new_v4(),
            AgentEventType::TaskCompleted,
            serde_json::json!({"result": "ok"}),
        )
        .with_state_transition(AgentState::Running, AgentState::Running)
        .with_message("Done")
        .with_severity(Severity::Info);

        assert_eq!(event.event_type, AgentEventType::StateTransition);
        assert_eq!(event.from_state, Some(AgentState::Running));
    }

    #[test]
    fn test_task_assignment_lifecycle() {
        let mut task = TaskAssignment::new(
            Uuid::new_v4(),
            "orchestrator",
            TaskType::Scan,
            serde_json::json!({"url": "https://example.com"}),
        );

        assert_eq!(task.status, TaskStatus::Pending);
        assert!(task.can_retry() == false);

        task.mark_started();
        assert_eq!(task.status, TaskStatus::InProgress);
        assert!(task.started_at.is_some());

        task.mark_completed(serde_json::json!({"findings": 5}));
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(task.result.is_some());
    }

    #[test]
    fn test_task_assignment_failed_can_retry() {
        let mut task = TaskAssignment::new(
            Uuid::new_v4(),
            "orchestrator",
            TaskType::Analyze,
            serde_json::json!({}),
        ).with_max_retries(5);

        task.mark_started();
        task.mark_failed("timeout");
        assert!(task.can_retry());
    }

    #[test]
    fn test_task_assignment_overdue() {
        let deadline = Utc::now() - chrono::Duration::hours(1);
        let task = TaskAssignment::new(
            Uuid::new_v4(),
            "orchestrator",
            TaskType::Report,
            serde_json::json!({}),
        ).with_deadline(deadline);

        assert!(task.is_overdue());
    }

    #[test]
    fn test_heartbeat_stale() {
        let mut hb = Heartbeat::new(Uuid::new_v4(), AgentState::Running);
        assert!(!hb.is_stale(30));

        hb.timestamp = Utc::now() - chrono::Duration::seconds(60);
        assert!(hb.is_stale(30));
    }

    #[test]
    fn test_agent_registry_entry_stale() {
        let identity = AgentIdentity::new(AgentType::Reporting, "reporter-1");
        let mut entry = AgentRegistryEntry::new(identity);
        assert!(!entry.is_stale(60));

        entry.last_updated = Utc::now() - chrono::Duration::seconds(120);
        assert!(entry.is_stale(60));
    }

    #[test]
    fn test_agent_registry_entry_needs_restart() {
        let identity = AgentIdentity::new(AgentType::ModelIntegration, "model-1");
        let mut entry = AgentRegistryEntry::new(identity);
        assert!(!entry.needs_restart());

        entry.status.state = AgentState::Error;
        assert!(entry.needs_restart());

        entry.enabled = false;
        assert!(!entry.needs_restart());
    }
}
