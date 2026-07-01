// shared/contracts/orchestrator_contract.go
// IWS v1.0 - Orchestrator Contract
// Mendefinisikan kontrak formal untuk orchestrator workflow management

package contracts

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
	"time"

	"github.com/google/uuid"
)

// ============================================================
// WORKFLOW STATES & TRANSITIONS
// ============================================================

// WorkflowState merepresentasikan state dalam orchestrator state machine
type WorkflowState string

const (
	StateIdle        WorkflowState = "IDLE"
	StatePreparing   WorkflowState = "PREPARING"
	StateScanning    WorkflowState = "SCANNING"
	StateAnalyzing   WorkflowState = "ANALYZING"
	StateReporting   WorkflowState = "REPORTING"
	StateComplete    WorkflowState = "COMPLETE"
	StateError       WorkflowState = "ERROR"
	StatePaused      WorkflowState = "PAUSED"
	StateCancelled   WorkflowState = "CANCELLED"
)

// ValidTransitions mendefinisikan transisi state yang valid
var ValidTransitions = map[WorkflowState][]WorkflowState{
	StateIdle:      {StatePreparing},
	StatePreparing: {StateScanning, StateError, StateCancelled},
	StateScanning:  {StatePaused, StateAnalyzing, StateError, StateCancelled},
	StatePaused:    {StateScanning, StateError, StateCancelled},
	StateAnalyzing: {StateReporting, StateError, StateCancelled},
	StateReporting: {StateComplete, StateError, StateCancelled},
	StateComplete:  {},
	StateError:     {StateIdle, StateCancelled},
	StateCancelled: {StateIdle},
}

// IsValidTransition memeriksa apakah transisi state valid
func IsValidTransition(from, to WorkflowState) bool {
	validTargets, exists := ValidTransitions[from]
	if !exists {
		return false
	}
	for _, target := range validTargets {
		if target == to {
			return true
		}
	}
	return false
}

// String mengembalikan representasi string dari WorkflowState
func (s WorkflowState) String() string {
	return string(s)
}

// IsTerminal mengembalikan true jika state adalah state terminal
func (s WorkflowState) IsTerminal() bool {
	return s == StateComplete || s == StateError || s == StateCancelled
}

// IsActive mengembalikan true jika workflow sedang berjalan
func (s WorkflowState) IsActive() bool {
	return s == StatePreparing || s == StateScanning || s == StateAnalyzing || s == StateReporting
}

// ============================================================
// WORKFLOW ID & TYPES
// ============================================================

// WorkflowID adalah unique identifier untuk workflow
type WorkflowID = uuid.UUID

// NewWorkflowID membuat WorkflowID baru
func NewWorkflowID() WorkflowID {
	return uuid.New()
}

// ParseWorkflowID mengkonversi string ke WorkflowID
func ParseWorkflowID(s string) (WorkflowID, error) {
	return uuid.Parse(s)
}

// ============================================================
// WORKFLOW REQUEST
// ============================================================

// WorkflowRequest merepresentasikan request untuk memulai workflow baru
type WorkflowRequest struct {
	ID            WorkflowID              `json:"id"`
	TargetURL     string                  `json:"target_url"`
	Profile       string                  `json:"profile"`
	Options       WorkflowOptions         `json:"options"`
	Tags          []string                `json:"tags"`
	Priority      WorkflowPriority        `json:"priority"`
	CallbackURL   string                  `json:"callback_url,omitempty"`
	Metadata      map[string]interface{}  `json:"metadata,omitempty"`
	CreatedAt     time.Time               `json:"created_at"`
	CreatedBy     string                  `json:"created_by"`
}

// WorkflowOptions mendefinisikan opsi untuk workflow
type WorkflowOptions struct {
	Timeout           time.Duration `json:"timeout"`
	MaxPages          int           `json:"max_pages"`
	FollowRedirects   bool          `json:"follow_redirects"`
	RespectRobots     bool          `json:"respect_robots"`
	EnableJS          bool          `json:"enable_js"`
	ScanDepth         int           `json:"scan_depth"`
	ThreadCount       int           `json:"thread_count"`
	DelayMS           int           `json:"delay_ms"`
	RetryCount        int           `json:"retry_count"`
	Modules           []string      `json:"modules"`
	ExcludeModules    []string      `json:"exclude_modules"`
	OutputFormats     []string      `json:"output_formats"`
	NotifyOnComplete  bool          `json:"notify_on_complete"`
	NotifyOnError     bool          `json:"notify_on_error"`
}

// DefaultWorkflowOptions mengembalikan opsi default
func DefaultWorkflowOptions() WorkflowOptions {
	return WorkflowOptions{
		Timeout:          30 * time.Minute,
		MaxPages:         500,
		FollowRedirects:  true,
		RespectRobots:    true,
		EnableJS:         false,
		ScanDepth:        3,
		ThreadCount:      50,
		DelayMS:          100,
		RetryCount:       3,
		Modules:          []string{},
		ExcludeModules:   []string{},
		OutputFormats:    []string{"json", "html"},
		NotifyOnComplete: false,
		NotifyOnError:    true,
	}
}

// Validate memvalidasi WorkflowOptions
func (o *WorkflowOptions) Validate() error {
	if o.Timeout < 1*time.Minute || o.Timeout > 2*time.Hour {
		return fmt.Errorf("timeout must be between 1 minute and 2 hours, got %v", o.Timeout)
	}
	if o.MaxPages < 1 || o.MaxPages > 10000 {
		return fmt.Errorf("max_pages must be between 1 and 10000, got %d", o.MaxPages)
	}
	if o.ScanDepth < 1 || o.ScanDepth > 10 {
		return fmt.Errorf("scan_depth must be between 1 and 10, got %d", o.ScanDepth)
	}
	if o.ThreadCount < 1 || o.ThreadCount > 200 {
		return fmt.Errorf("thread_count must be between 1 and 200, got %d", o.ThreadCount)
	}
	if o.DelayMS < 0 || o.DelayMS > 10000 {
		return fmt.Errorf("delay_ms must be between 0 and 10000, got %d", o.DelayMS)
	}
	if o.RetryCount > 10 {
		return fmt.Errorf("retry_count must be between 0 and 10, got %d", o.RetryCount)
	}
	return nil
}

// WorkflowPriority merepresentasikan prioritas workflow
type WorkflowPriority int

const (
	PriorityLow      WorkflowPriority = 0
	PriorityNormal   WorkflowPriority = 1
	PriorityHigh     WorkflowPriority = 2
	PriorityCritical WorkflowPriority = 3
)

// String mengembalikan representasi string dari WorkflowPriority
func (p WorkflowPriority) String() string {
	switch p {
	case PriorityLow:
		return "low"
	case PriorityNormal:
		return "normal"
	case PriorityHigh:
		return "high"
	case PriorityCritical:
		return "critical"
	default:
		return "unknown"
	}
}

// ============================================================
// WORKFLOW STATUS
// ============================================================

// WorkflowStatus merepresentasikan status terkini dari workflow
type WorkflowStatus struct {
	ID              WorkflowState           `json:"id"`
	State           WorkflowState           `json:"state"`
	Progress        float64                 `json:"progress"`
	CurrentStep     string                  `json:"current_step"`
	StartTime       time.Time               `json:"start_time"`
	EndTime         *time.Time              `json:"end_time,omitempty"`
	ElapsedTime     time.Duration           `json:"elapsed_time"`
	EstimatedTime   time.Duration           `json:"estimated_time"`
	Error           string                  `json:"error,omitempty"`
	ErrorCount      int                     `json:"error_count"`
	WarningCount    int                     `json:"warning_count"`
	ModulesDone     []string                `json:"modules_done"`
	ModulesPending  []string                `json:"modules_pending"`
	ModulesFailed   []string                `json:"modules_failed"`
	PagesDone       int                     `json:"pages_done"`
	PagesTotal      int                     `json:"pages_total"`
	FindingsCount   int                     `json:"findings_count"`
	Metadata        map[string]interface{}  `json:"metadata,omitempty"`
	UpdatedAt       time.Time               `json:"updated_at"`
}

// NewWorkflowStatus membuat WorkflowStatus baru
func NewWorkflowStatus(id WorkflowID) WorkflowStatus {
	now := time.Now()
	return WorkflowStatus{
		ID:             StateIdle,
		State:          StateIdle,
		Progress:       0,
		CurrentStep:    "initializing",
		StartTime:      now,
		ElapsedTime:    0,
		EstimatedTime:  30 * time.Minute,
		ErrorCount:     0,
		WarningCount:   0,
		ModulesDone:    []string{},
		ModulesPending: []string{},
		ModulesFailed:  []string{},
		PagesDone:      0,
		PagesTotal:     0,
		FindingsCount:  0,
		Metadata:       make(map[string]interface{}),
		UpdatedAt:      now,
	}
}

// UpdateProgress mengupdate progress workflow
func (ws *WorkflowStatus) UpdateProgress(progress float64, currentStep string) {
	ws.Progress = progress
	ws.CurrentStep = currentStep
	ws.UpdatedAt = time.Now()
	ws.ElapsedTime = time.Since(ws.StartTime)

	if progress > 0 && ws.ElapsedTime > 0 {
		totalEstimated := float64(ws.ElapsedTime) / (progress / 100.0)
		remaining := time.Duration(totalEstimated) - ws.ElapsedTime
		ws.EstimatedTime = remaining
	}
}

// AddModuleDone menambahkan module yang selesai
func (ws *WorkflowStatus) AddModuleDone(moduleName string) {
	ws.ModulesDone = append(ws.ModulesDone, moduleName)
	ws.UpdatedAt = time.Now()
}

// AddModuleFailed menambahkan module yang gagal
func (ws *WorkflowStatus) AddModuleFailed(moduleName string, err error) {
	ws.ModulesFailed = append(ws.ModulesFailed, moduleName)
	ws.ErrorCount++
	if err != nil {
		ws.Error = err.Error()
	}
	ws.UpdatedAt = time.Now()
}

// SetState mengubah state workflow dengan validasi transisi
func (ws *WorkflowStatus) SetState(newState WorkflowState) error {
	if !IsValidTransition(ws.State, newState) {
		return fmt.Errorf("invalid state transition from %s to %s", ws.State, newState)
	}
	ws.State = newState
	ws.UpdatedAt = time.Now()

	if newState.IsTerminal() {
		now := time.Now()
		ws.EndTime = &now
		ws.Progress = 100
	}

	return nil
}

// ToJSON mengkonversi WorkflowStatus ke JSON
func (ws *WorkflowStatus) ToJSON() ([]byte, error) {
	return json.Marshal(ws)
}

// FromJSON mengkonversi JSON ke WorkflowStatus
func FromJSON(data []byte) (*WorkflowStatus, error) {
	var ws WorkflowStatus
	err := json.Unmarshal(data, &ws)
	if err != nil {
		return nil, err
	}
	return &ws, nil
}

// ============================================================
// WORKFLOW HISTORY
// ============================================================

// WorkflowEvent merepresentasikan event dalam workflow
type WorkflowEvent struct {
	ID          uuid.UUID     `json:"id"`
	WorkflowID  WorkflowID   `json:"workflow_id"`
	EventType   string        `json:"event_type"`
	FromState   WorkflowState `json:"from_state"`
	ToState     WorkflowState `json:"to_state"`
	Message     string        `json:"message"`
	Details     interface{}   `json:"details,omitempty"`
	Timestamp   time.Time     `json:"timestamp"`
}

// WorkflowHistory menyimpan riwayat workflow
type WorkflowHistory struct {
	mu      sync.RWMutex
	Events  []WorkflowEvent        `json:"events"`
	ByID    map[WorkflowID][]WorkflowEvent `json:"-"`
	MaxSize int                    `json:"-"`
}

// NewWorkflowHistory membuat WorkflowHistory baru
func NewWorkflowHistory(maxSize int) *WorkflowHistory {
	return &WorkflowHistory{
		Events:  make([]WorkflowEvent, 0, maxSize),
		ByID:    make(map[WorkflowID][]WorkflowEvent),
		MaxSize: maxSize,
	}
}

// AddEvent menambahkan event ke history
func (wh *WorkflowHistory) AddEvent(event WorkflowEvent) {
	wh.mu.Lock()
	defer wh.mu.Unlock()

	wh.Events = append(wh.Events, event)
	wh.ByID[event.WorkflowID] = append(wh.ByID[event.WorkflowID], event)

	if len(wh.Events) > wh.MaxSize {
		excess := len(wh.Events) - wh.MaxSize
		wh.Events = wh.Events[excess:]
	}
}

// GetEventsByWorkflow mengembalikan event untuk workflow tertentu
func (wh *WorkflowHistory) GetEventsByWorkflow(id WorkflowID) []WorkflowEvent {
	wh.mu.RLock()
	defer wh.mu.RUnlock()

	events, exists := wh.ByID[id]
	if !exists {
		return []WorkflowEvent{}
	}
	result := make([]WorkflowEvent, len(events))
	copy(result, events)
	return result
}

// GetRecentEvents mengembalikan event terbaru
func (wh *WorkflowHistory) GetRecentEvents(limit int) []WorkflowEvent {
	wh.mu.RLock()
	defer wh.mu.RUnlock()

	if limit > len(wh.Events) {
		limit = len(wh.Events)
	}
	start := len(wh.Events) - limit
	result := make([]WorkflowEvent, limit)
	copy(result, wh.Events[start:])
	return result
}

// ============================================================
// CALLBACK & EVENT HANDLING
// ============================================================

// WorkflowCallback adalah fungsi callback untuk event workflow
type WorkflowCallback func(status WorkflowStatus)

// CallbackManager mengelola callback subscriptions
type CallbackManager struct {
	mu          sync.RWMutex
	callbacks   map[string][]WorkflowCallback
	globalCallbacks []WorkflowCallback
}

// NewCallbackManager membuat CallbackManager baru
func NewCallbackManager() *CallbackManager {
	return &CallbackManager{
		callbacks:       make(map[string][]WorkflowCallback),
		globalCallbacks: make([]WorkflowCallback, 0),
	}
}

// On mendaftarkan callback untuk event tertentu
func (cm *CallbackManager) On(event string, callback WorkflowCallback) {
	cm.mu.Lock()
	defer cm.mu.Unlock()

	cm.callbacks[event] = append(cm.callbacks[event], callback)
}

// OnAll mendaftarkan callback untuk semua event
func (cm *CallbackManager) OnAll(callback WorkflowCallback) {
	cm.mu.Lock()
	defer cm.mu.Unlock()

	cm.globalCallbacks = append(cm.globalCallbacks, callback)
}

// Emit memanggil semua callback yang terdaftar untuk event tertentu
func (cm *CallbackManager) Emit(event string, status WorkflowStatus) {
	cm.mu.RLock()
	callbacks := make([]WorkflowCallback, 0)
	if cbs, exists := cm.callbacks[event]; exists {
		callbacks = append(callbacks, cbs...)
	}
	callbacks = append(callbacks, cm.globalCallbacks...)
	cm.mu.RUnlock()

	for _, cb := range callbacks {
		go func(callback WorkflowCallback, s WorkflowStatus) {
			defer func() {
				if r := recover(); r != nil {
					// Log panic recovery
					_ = r
				}
			}()
			callback(s)
		}(cb, status)
	}
}

// RemoveAll menghapus semua callback
func (cm *CallbackManager) RemoveAll() {
	cm.mu.Lock()
	defer cm.mu.Unlock()

	cm.callbacks = make(map[string][]WorkflowCallback)
	cm.globalCallbacks = make([]WorkflowCallback, 0)
}

// ============================================================
// WORKFLOW MANAGER INTERFACE
// ============================================================

// OrchestratorContract mendefinisikan interface untuk orchestrator
type OrchestratorContract interface {
	// StartWorkflow memulai workflow baru
	// Returns: WorkflowID jika berhasil, error jika gagal
	StartWorkflow(ctx context.Context, req WorkflowRequest) (WorkflowID, error)

	// GetWorkflowStatus mendapatkan status workflow
	GetWorkflowStatus(ctx context.Context, id WorkflowID) (WorkflowStatus, error)

	// PauseWorkflow menjeda workflow yang sedang berjalan
	PauseWorkflow(ctx context.Context, id WorkflowID) error

	// ResumeWorkflow melanjutkan workflow yang dijeda
	ResumeWorkflow(ctx context.Context, id WorkflowID) error

	// CancelWorkflow membatalkan workflow
	CancelWorkflow(ctx context.Context, id WorkflowID) error

	// GetWorkflowHistory mendapatkan riwayat status workflow
	GetWorkflowHistory(ctx context.Context, id WorkflowID) ([]WorkflowStatus, error)

	// ListActiveWorkflows mendapatkan semua workflow yang aktif
	ListActiveWorkflows(ctx context.Context) ([]WorkflowID, error)

	// ListAllWorkflows mendapatkan semua workflow
	ListAllWorkflows(ctx context.Context, limit, offset int) ([]WorkflowStatus, error)

	// RegisterCallback mendaftarkan callback untuk event workflow
	RegisterCallback(event string, callback WorkflowCallback)

	// GetWorkflowCount mendapatkan jumlah workflow
	GetWorkflowCount(ctx context.Context) (int, error)

	// GetWorkflowMetrics mendapatkan metrics orchestrator
	GetWorkflowMetrics(ctx context.Context) (OrchestratorMetrics, error)

	// Shutdown menghentikan orchestrator dengan graceful shutdown
	Shutdown(ctx context.Context) error
}

// ============================================================
// ORCHESTRATOR METRICS
// ============================================================

// OrchestratorMetrics merepresentasikan metrics orchestrator
type OrchestratorMetrics struct {
	TotalWorkflows      int64                  `json:"total_workflows"`
	ActiveWorkflows     int64                  `json:"active_workflows"`
	CompletedWorkflows  int64                  `json:"completed_workflows"`
	FailedWorkflows     int64                  `json:"failed_workflows"`
	CancelledWorkflows  int64                  `json:"cancelled_workflows"`
	AverageDuration     time.Duration          `json:"average_duration"`
	TotalScans          int64                  `json:"total_scans"`
	TotalFindings       int64                  `json:"total_findings"`
	StateDistribution   map[WorkflowState]int64 `json:"state_distribution"`
	ErrorRate           float64                `json:"error_rate"`
	Uptime              time.Duration          `json:"uptime"`
	LastWorkflowTime    time.Time              `json:"last_workflow_time"`
}

// ============================================================
// WORKFLOW CONFIGURATION
// ============================================================

// OrchestratorConfig menyimpan konfigurasi orchestrator
type OrchestratorConfig struct {
	MaxConcurrentWorkflows int           `json:"max_concurrent_workflows"`
	MaxWorkflowDuration    time.Duration `json:"max_workflow_duration"`
	MaxQueueSize           int           `json:"max_queue_size"`
	HistorySize            int           `json:"history_size"`
	EnablePersistence      bool          `json:"enable_persistence"`
	PersistencePath        string        `json:"persistence_path"`
	AutoRetryFailed        bool          `json:"auto_retry_failed"`
	MaxRetries             int           `json:"max_retries"`
	RetryDelay             time.Duration `json:"retry_delay"`
	HealthCheckInterval    time.Duration `json:"health_check_interval"`
	CleanupInterval        time.Duration `json:"cleanup_interval"`
}

// DefaultOrchestratorConfig mengembalikan konfigurasi default
func DefaultOrchestratorConfig() OrchestratorConfig {
	return OrchestratorConfig{
		MaxConcurrentWorkflows: 10,
		MaxWorkflowDuration:    2 * time.Hour,
		MaxQueueSize:           1000,
		HistorySize:            10000,
		EnablePersistence:      true,
		PersistencePath:        "./data/orchestrator.db",
		AutoRetryFailed:        true,
		MaxRetries:             3,
		RetryDelay:             5 * time.Minute,
		HealthCheckInterval:    30 * time.Second,
		CleanupInterval:        1 * time.Hour,
	}
}

// Validate memvalidasi konfigurasi orchestrator
func (c *OrchestratorConfig) Validate() error {
	if c.MaxConcurrentWorkflows < 1 || c.MaxConcurrentWorkflows > 100 {
		return fmt.Errorf("max_concurrent_workflows must be 1-100, got %d", c.MaxConcurrentWorkflows)
	}
	if c.MaxWorkflowDuration < 1*time.Minute {
		return fmt.Errorf("max_workflow_duration must be at least 1 minute")
	}
	if c.MaxQueueSize < 10 || c.MaxQueueSize > 10000 {
		return fmt.Errorf("max_queue_size must be 10-10000, got %d", c.MaxQueueSize)
	}
	if c.MaxRetries > 20 {
		return fmt.Errorf("max_retries must be 0-20, got %d", c.MaxRetries)
	}
	return nil
}

// ============================================================
// WORKFLOW QUEUE
// ============================================================

// WorkflowQueue adalah priority queue untuk workflow
type WorkflowQueue struct {
	mu       sync.Mutex
	high     []WorkflowRequest
	normal   []WorkflowRequest
	low      []WorkflowRequest
	maxSize  int
}

// NewWorkflowQueue membuat WorkflowQueue baru
func NewWorkflowQueue(maxSize int) *WorkflowQueue {
	return &WorkflowQueue{
		high:    make([]WorkflowRequest, 0),
		normal:  make([]WorkflowRequest, 0),
		low:     make([]WorkflowRequest, 0),
		maxSize: maxSize,
	}
}

// Enqueue menambahkan workflow ke queue berdasarkan prioritas
func (q *WorkflowQueue) Enqueue(req WorkflowRequest) error {
	q.mu.Lock()
	defer q.mu.Unlock()

	totalSize := len(q.high) + len(q.normal) + len(q.low)
	if totalSize >= q.maxSize {
		return fmt.Errorf("queue is full: %d/%d", totalSize, q.maxSize)
	}

	switch req.Priority {
	case PriorityCritical, PriorityHigh:
		q.high = append(q.high, req)
	case PriorityNormal:
		q.normal = append(q.normal, req)
	case PriorityLow:
		q.low = append(q.low, req)
	default:
		q.normal = append(q.normal, req)
	}

	return nil
}

// Dequeue mengambil workflow berikutnya dari queue (prioritas tertinggi)
func (q *WorkflowQueue) Dequeue() (*WorkflowRequest, error) {
	q.mu.Lock()
	defer q.mu.Unlock()

	if len(q.high) > 0 {
		req := q.high[0]
		q.high = q.high[1:]
		return &req, nil
	}

	if len(q.normal) > 0 {
		req := q.normal[0]
		q.normal = q.normal[1:]
		return &req, nil
	}

	if len(q.low) > 0 {
		req := q.low[0]
		q.low = q.low[1:]
		return &req, nil
	}

	return nil, fmt.Errorf("queue is empty")
}

// Size mengembalikan jumlah item dalam queue
func (q *WorkflowQueue) Size() int {
	q.mu.Lock()
	defer q.mu.Unlock()
	return len(q.high) + len(q.normal) + len(q.low)
}

// Clear mengosongkan queue
func (q *WorkflowQueue) Clear() {
	q.mu.Lock()
	defer q.mu.Unlock()

	q.high = make([]WorkflowRequest, 0)
	q.normal = make([]WorkflowRequest, 0)
	q.low = make([]WorkflowRequest, 0)
}

// ============================================================
// WORKFLOW STORE INTERFACE
// ============================================================

// WorkflowStore mendefinisikan interface untuk penyimpanan workflow
type WorkflowStore interface {
	SaveWorkflow(ctx context.Context, id WorkflowID, status WorkflowStatus) error
	LoadWorkflow(ctx context.Context, id WorkflowID) (WorkflowStatus, error)
	DeleteWorkflow(ctx context.Context, id WorkflowID) error
	ListWorkflows(ctx context.Context, filter WorkflowFilter) ([]WorkflowStatus, error)
	SaveEvent(ctx context.Context, event WorkflowEvent) error
	LoadEvents(ctx context.Context, workflowID WorkflowID) ([]WorkflowEvent, error)
	Close() error
}

// WorkflowFilter mendefinisikan filter untuk query workflow
type WorkflowFilter struct {
	State     *WorkflowState `json:"state,omitempty"`
	Priority  *WorkflowPriority `json:"priority,omitempty"`
	Since     *time.Time     `json:"since,omitempty"`
	Until     *time.Time     `json:"until,omitempty"`
	Tags      []string       `json:"tags,omitempty"`
	Limit     int            `json:"limit"`
	Offset    int            `json:"offset"`
}

// ============================================================
// CONTEXT EXTENSIONS
// ============================================================

// Context key types untuk context values
type contextKey string

const (
	ContextKeyWorkflowID contextKey = "workflow_id"
	ContextKeyUserID     contextKey = "user_id"
	ContextKeyTraceID    contextKey = "trace_id"
)

// WithWorkflowID menambahkan WorkflowID ke context
func WithWorkflowID(ctx context.Context, id WorkflowID) context.Context {
	return context.WithValue(ctx, ContextKeyWorkflowID, id)
}

// GetWorkflowIDFromContext mengambil WorkflowID dari context
func GetWorkflowIDFromContext(ctx context.Context) (WorkflowID, bool) {
	id, ok := ctx.Value(ContextKeyWorkflowID).(WorkflowID)
	return id, ok
}

// WithUserID menambahkan UserID ke context
func WithUserID(ctx context.Context, userID string) context.Context {
	return context.WithValue(ctx, ContextKeyUserID, userID)
}

// GetUserIDFromContext mengambil UserID dari context
func GetUserIDFromContext(ctx context.Context) (string, bool) {
	userID, ok := ctx.Value(ContextKeyUserID).(string)
	return userID, ok
}

// ============================================================
// ERRORS
// ============================================================

// OrchestratorError merepresentasikan error dari orchestrator
type OrchestratorError struct {
	Code      string `json:"code"`
	Message   string `json:"message"`
	WorkflowID WorkflowID `json:"workflow_id,omitempty"`
	State     WorkflowState `json:"state,omitempty"`
	Severity  string `json:"severity"`
	Retryable bool   `json:"retryable"`
}

// Error mengimplementasikan error interface
func (e *OrchestratorError) Error() string {
	if e.WorkflowID != uuid.Nil {
		return fmt.Sprintf("[%s] %s (workflow: %s, state: %s)", e.Code, e.Message, e.WorkflowID, e.State)
	}
	return fmt.Sprintf("[%s] %s", e.Code, e.Message)
}

// Predefined error constructors
func ErrWorkflowNotFound(id WorkflowID) *OrchestratorError {
	return &OrchestratorError{
		Code:      "O3001",
		Message:   "workflow not found",
		WorkflowID: id,
		Severity:  "medium",
		Retryable: false,
	}
}

func ErrWorkflowAlreadyRunning(id WorkflowID) *OrchestratorError {
	return &OrchestratorError{
		Code:      "O3002",
		Message:   "workflow already running",
		WorkflowID: id,
		Severity:  "low",
		Retryable: false,
	}
}

func ErrWorkflowPaused(id WorkflowID) *OrchestratorError {
	return &OrchestratorError{
		Code:      "O3003",
		Message:   "workflow is paused",
		WorkflowID: id,
		State:     StatePaused,
		Severity:  "low",
		Retryable: false,
	}
}

func ErrWorkflowCancelled(id WorkflowID) *OrchestratorError {
	return &OrchestratorError{
		Code:      "O3004",
		Message:   "workflow has been cancelled",
		WorkflowID: id,
		State:     StateCancelled,
		Severity:  "low",
		Retryable: false,
	}
}

func ErrInvalidTransition(id WorkflowID, from, to WorkflowState) *OrchestratorError {
	return &OrchestratorError{
		Code:      "O3005",
		Message:   fmt.Sprintf("invalid state transition from %s to %s", from, to),
		WorkflowID: id,
		State:     from,
		Severity:  "high",
		Retryable: false,
	}
}

func ErrQueueFull(currentSize, maxSize int) *OrchestratorError {
	return &OrchestratorError{
		Code:      "O3006",
		Message:   fmt.Sprintf("workflow queue is full: %d/%d", currentSize, maxSize),
		Severity:  "medium",
		Retryable: true,
	}
}

func ErrMaxConcurrentReached(current, max int) *OrchestratorError {
	return &OrchestratorError{
		Code:      "O3007",
		Message:   fmt.Sprintf("max concurrent workflows reached: %d/%d", current, max),
		Severity:  "medium",
		Retryable: true,
	}
}

func ErrWorkflowTimeout(id WorkflowID, duration time.Duration) *OrchestratorError {
	return &OrchestratorError{
		Code:      "O3008",
		Message:   fmt.Sprintf("workflow timed out after %v", duration),
		WorkflowID: id,
		Severity:  "high",
		Retryable: true,
	}
}

func ErrInternalError(msg string) *OrchestratorError {
	return &OrchestratorError{
		Code:      "O3009",
		Message:   msg,
		Severity:  "critical",
		Retryable: false,
	}
}

func ErrPersistenceFailed(msg string) *OrchestratorError {
	return &OrchestratorError{
		Code:      "O3010",
		Message:   fmt.Sprintf("persistence operation failed: %s", msg),
		Severity:  "high",
		Retryable: true,
	}
}

// ============================================================
// UNIT TESTS
// ============================================================
// Untuk menjalankan: go test ./shared/contracts/
