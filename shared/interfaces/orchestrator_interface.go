// shared/interfaces/orchestrator_interface.go
// IWS v1.0 - Orchestrator Interface
// Mendefinisikan interface Orchestrator untuk workflow management

package interfaces

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/google/uuid"
)

// ============================================================
// ORCHESTRATOR INTERFACE
// ============================================================

// Orchestrator mendefinisikan interface untuk workflow orchestration
type Orchestrator interface {
	// StartWorkflow memulai workflow baru
	StartWorkflow(ctx context.Context, req WorkflowRequest) (uuid.UUID, error)

	// GetWorkflowStatus mendapatkan status workflow
	GetWorkflowStatus(ctx context.Context, id uuid.UUID) (WorkflowStatus, error)

	// PauseWorkflow menjeda workflow
	PauseWorkflow(ctx context.Context, id uuid.UUID) error

	// ResumeWorkflow melanjutkan workflow yang dijeda
	ResumeWorkflow(ctx context.Context, id uuid.UUID) error

	// CancelWorkflow membatalkan workflow
	CancelWorkflow(ctx context.Context, id uuid.UUID) error

	// GetWorkflowHistory mendapatkan riwayat status workflow
	GetWorkflowHistory(ctx context.Context, id uuid.UUID) ([]WorkflowStatus, error)

	// ListActiveWorkflows mendapatkan semua workflow yang aktif
	ListActiveWorkflows(ctx context.Context) ([]uuid.UUID, error)

	// ListAllWorkflows mendapatkan semua workflow
	ListAllWorkflows(ctx context.Context, limit, offset int) ([]WorkflowStatus, error)

	// On mendaftarkan callback untuk event workflow (SPEC FIX #4)
	// event: nama event (contoh: "scan_started", "scan_completed")
	// callback: fungsi yang dipanggil saat event terjadi
	On(event string, callback func(WorkflowStatus))

	// RegisterCallback mendaftarkan callback untuk event workflow (alias untuk On)
	RegisterCallback(event string, callback WorkflowCallback)

	// GetWorkflowCount mendapatkan jumlah workflow
	GetWorkflowCount(ctx context.Context) (int, error)

	// GetWorkflowMetrics mendapatkan metrics orchestrator
	GetWorkflowMetrics(ctx context.Context) (OrchestratorMetrics, error)

	// Shutdown menghentikan orchestrator
	Shutdown(ctx context.Context) error

	// IsHealthy mengecek health orchestrator
	IsHealthy() bool

	// GetUptime mendapatkan uptime orchestrator
	GetUptime() time.Duration
}

// ============================================================
// WORKFLOW STATES & TYPES
// ============================================================

// WorkflowState merepresentasikan state workflow
type WorkflowState string

const (
	StateIdle      WorkflowState = "IDLE"
	StatePreparing WorkflowState = "PREPARING"
	StateScanning  WorkflowState = "SCANNING"
	StateAnalyzing WorkflowState = "ANALYZING"
	StateReporting WorkflowState = "REPORTING"
	StateComplete  WorkflowState = "COMPLETE"
	StateFailed    WorkflowState = "FAILED"
	StateCancelled WorkflowState = "CANCELLED"
	StatePaused    WorkflowState = "PAUSED"
)

// IsTerminal mengembalikan true jika state adalah terminal
func (s WorkflowState) IsTerminal() bool {
	return s == StateComplete || s == StateFailed || s == StateCancelled
}

// IsActive mengembalikan true jika workflow sedang berjalan
func (s WorkflowState) IsActive() bool {
	return s == StatePreparing || s == StateScanning || s == StateAnalyzing || s == StateReporting
}

// String mengembalikan representasi string
func (s WorkflowState) String() string {
	return string(s)
}

// ============================================================
// WORKFLOW PRIORITY
// ============================================================

// WorkflowPriority merepresentasikan prioritas workflow
type WorkflowPriority int

const (
	PriorityLow      WorkflowPriority = 0
	PriorityNormal   WorkflowPriority = 1
	PriorityHigh     WorkflowPriority = 2
	PriorityCritical WorkflowPriority = 3
)

// String mengembalikan representasi string
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
// WORKFLOW REQUEST
// ============================================================

// WorkflowRequest merepresentasikan request workflow
type WorkflowRequest struct {
	ID          uuid.UUID              `json:"id"`
	TargetURL   string                 `json:"target_url"`
	Profile     string                 `json:"profile"`
	Options     WorkflowOptions        `json:"options"`
	Tags        []string               `json:"tags"`
	Priority    WorkflowPriority       `json:"priority"`
	CallbackURL string                 `json:"callback_url,omitempty"`
	Metadata    map[string]interface{} `json:"metadata,omitempty"`
	CreatedAt   time.Time              `json:"created_at"`
	CreatedBy   string                 `json:"created_by"`
}

// NewWorkflowRequest membuat WorkflowRequest baru
func NewWorkflowRequest(targetURL, profile, createdBy string) WorkflowRequest {
	return WorkflowRequest{
		ID:        uuid.New(),
		TargetURL: targetURL,
		Profile:   profile,
		Options:   DefaultWorkflowOptions(),
		Priority:  PriorityNormal,
		CreatedAt: time.Now(),
		CreatedBy: createdBy,
	}
}

// WithPriority set priority
func (r WorkflowRequest) WithPriority(p WorkflowPriority) WorkflowRequest {
	r.Priority = p
	return r
}

// WithOptions set options
func (r WorkflowRequest) WithOptions(o WorkflowOptions) WorkflowRequest {
	r.Options = o
	return r
}

// WithTags set tags
func (r WorkflowRequest) WithTags(tags []string) WorkflowRequest {
	r.Tags = tags
	return r
}

// Validate validasi request
func (r *WorkflowRequest) Validate() error {
	if r.TargetURL == "" {
		return fmt.Errorf("target_url is required")
	}
	if r.Profile == "" {
		r.Profile = "moderate"
	}
	if r.Priority < PriorityLow || r.Priority > PriorityCritical {
		return fmt.Errorf("invalid priority: %d", r.Priority)
	}
	return r.Options.Validate()
}

// ============================================================
// WORKFLOW OPTIONS
// ============================================================

// WorkflowOptions mendefinisikan opsi workflow
type WorkflowOptions struct {
	Timeout          time.Duration `json:"timeout"`
	MaxPages         int           `json:"max_pages"`
	FollowRedirects  bool          `json:"follow_redirects"`
	RespectRobots    bool          `json:"respect_robots"`
	EnableJS         bool          `json:"enable_js"`
	ScanDepth        int           `json:"scan_depth"`
	ThreadCount      int           `json:"thread_count"`
	DelayMS          int           `json:"delay_ms"`
	RetryCount       int           `json:"retry_count"`
	Modules          []string      `json:"modules"`
	ExcludeModules   []string      `json:"exclude_modules"`
	OutputFormats    []string      `json:"output_formats"`
	NotifyOnComplete bool          `json:"notify_on_complete"`
	NotifyOnError    bool          `json:"notify_on_error"`
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

// Validate validasi opsi
func (o *WorkflowOptions) Validate() error {
	if o.Timeout < 1*time.Minute || o.Timeout > 2*time.Hour {
		return fmt.Errorf("timeout must be 1m-2h, got %v", o.Timeout)
	}
	if o.MaxPages < 1 || o.MaxPages > 10000 {
		return fmt.Errorf("max_pages must be 1-10000, got %d", o.MaxPages)
	}
	if o.ScanDepth < 1 || o.ScanDepth > 10 {
		return fmt.Errorf("scan_depth must be 1-10, got %d", o.ScanDepth)
	}
	if o.ThreadCount < 1 || o.ThreadCount > 200 {
		return fmt.Errorf("thread_count must be 1-200, got %d", o.ThreadCount)
	}
	if o.DelayMS < 0 || o.DelayMS > 10000 {
		return fmt.Errorf("delay_ms must be 0-10000, got %d", o.DelayMS)
	}
	if o.RetryCount > 10 {
		return fmt.Errorf("retry_count must be 0-10, got %d", o.RetryCount)
	}
	return nil
}

// ============================================================
// WORKFLOW STATUS
// ============================================================

// WorkflowStatus merepresentasikan status workflow
type WorkflowStatus struct {
	ID                uuid.UUID              `json:"id"`
	State             WorkflowState          `json:"state"`
	Progress          float64                `json:"progress"`
	CurrentStep       string                 `json:"current_step"`
	StartTime         time.Time              `json:"start_time"`
	EndTime           *time.Time             `json:"end_time,omitempty"`
	ElapsedTime       time.Duration          `json:"elapsed_time"`
	EstimatedTime     time.Duration          `json:"estimated_time"`
	Error             string                 `json:"error,omitempty"`
	ErrorCount        int                    `json:"error_count"`
	WarningCount      int                    `json:"warning_count"`
	ModulesDone       []string               `json:"modules_done"`
	ModulesPending    []string               `json:"modules_pending"`
	ModulesFailed     []string               `json:"modules_failed"`
	PagesDone         int                    `json:"pages_done"`
	PagesTotal        int                    `json:"pages_total"`
	FindingsCount     int                    `json:"findings_count"`
	Metadata          map[string]interface{} `json:"metadata,omitempty"`
	UpdatedAt         time.Time              `json:"updated_at"`
}

// NewWorkflowStatus membuat status baru
func NewWorkflowStatus(id uuid.UUID) WorkflowStatus {
	now := time.Now()
	return WorkflowStatus{
		ID:             id,
		State:          StateIdle,
		Progress:       0,
		CurrentStep:    "initializing",
		StartTime:      now,
		ElapsedTime:    0,
		EstimatedTime:  30 * time.Minute,
		ModulesDone:    []string{},
		ModulesPending: []string{},
		ModulesFailed:  []string{},
		Metadata:       make(map[string]interface{}),
		UpdatedAt:      now,
	}
}

// UpdateProgress mengupdate progress
func (ws *WorkflowStatus) UpdateProgress(progress float64, currentStep string) {
	if progress < 0 {
		progress = 0
	}
	if progress > 100 {
		progress = 100
	}
	ws.Progress = progress
	ws.CurrentStep = currentStep
	ws.UpdatedAt = time.Now()
	ws.ElapsedTime = time.Since(ws.StartTime)

	if progress > 0 && ws.ElapsedTime > 0 {
		totalEst := float64(ws.ElapsedTime) / (progress / 100.0)
		remaining := time.Duration(totalEst) - ws.ElapsedTime
		if remaining > 0 {
			ws.EstimatedTime = remaining
		}
	}
}

// AddModuleDone menambah module selesai
func (ws *WorkflowStatus) AddModuleDone(name string) {
	ws.ModulesDone = append(ws.ModulesDone, name)
	ws.UpdatedAt = time.Now()
}

// AddModuleFailed menambah module gagal
func (ws *WorkflowStatus) AddModuleFailed(name string, err error) {
	ws.ModulesFailed = append(ws.ModulesFailed, name)
	ws.ErrorCount++
	if err != nil {
		ws.Error = err.Error()
	}
	ws.UpdatedAt = time.Now()
}

// TransitionState transisi state dengan validasi
func (ws *WorkflowStatus) TransitionState(newState WorkflowState) error {
	validTransitions := map[WorkflowState][]WorkflowState{
		StateIdle:      {StatePreparing},
		StatePreparing: {StateScanning, StateFailed, StateCancelled},
		StateScanning:  {StatePaused, StateAnalyzing, StateFailed, StateCancelled},
		StatePaused:    {StateScanning, StateFailed, StateCancelled},
		StateAnalyzing: {StateReporting, StateFailed, StateCancelled},
		StateReporting: {StateComplete, StateFailed, StateCancelled},
		StateComplete:  {},
		StateFailed:    {StateIdle},
		StateCancelled: {StateIdle},
	}

	validTargets, exists := validTransitions[ws.State]
	if !exists {
		return fmt.Errorf("invalid current state: %s", ws.State)
	}

	for _, target := range validTargets {
		if target == newState {
			ws.State = newState
			ws.UpdatedAt = time.Now()
			if newState.IsTerminal() {
				now := time.Now()
				ws.EndTime = &now
				ws.Progress = 100
			}
			return nil
		}
	}

	return fmt.Errorf("invalid transition from %s to %s", ws.State, newState)
}

// ============================================================
// WORKFLOW CALLBACK
// ============================================================

// WorkflowCallback adalah fungsi callback untuk event workflow
type WorkflowCallback func(status WorkflowStatus)

// CallbackManager mengelola callback subscriptions
type CallbackManager struct {
	mu              sync.RWMutex
	callbacks       map[string][]WorkflowCallback
	globalCallbacks []WorkflowCallback
}

// NewCallbackManager membuat CallbackManager baru
func NewCallbackManager() *CallbackManager {
	return &CallbackManager{
		callbacks:       make(map[string][]WorkflowCallback),
		globalCallbacks: make([]WorkflowCallback, 0),
	}
}

// On mendaftarkan callback untuk event (SPEC FIX #4 — public API sesuai spec)
// event: nama event (contoh: "scan_started", "scan_completed", "module_started")
// callback: fungsi yang dipanggil saat event terjadi dengan parameter WorkflowStatus
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

// Emit memanggil semua callback untuk event
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
					// Log panic recovery — jangan crash karena satu callback
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

// Count mengembalikan jumlah callback terdaftar
func (cm *CallbackManager) Count() int {
	cm.mu.RLock()
	defer cm.mu.RUnlock()
	count := len(cm.globalCallbacks)
	for _, cbs := range cm.callbacks {
		count += len(cbs)
	}
	return count
}

// ============================================================
// ORCHESTRATOR METRICS
// ============================================================

// OrchestratorMetrics merepresentasikan metrics orchestrator
type OrchestratorMetrics struct {
	TotalWorkflows     int64                    `json:"total_workflows"`
	ActiveWorkflows    int64                    `json:"active_workflows"`
	CompletedWorkflows int64                    `json:"completed_workflows"`
	FailedWorkflows    int64                    `json:"failed_workflows"`
	CancelledWorkflows int64                    `json:"cancelled_workflows"`
	AverageDuration    time.Duration            `json:"average_duration"`
	TotalScans         int64                    `json:"total_scans"`
	TotalFindings      int64                    `json:"total_findings"`
	StateDistribution  map[WorkflowState]int64  `json:"state_distribution"`
	ErrorRate          float64                  `json:"error_rate"`
	Uptime             time.Duration            `json:"uptime"`
	LastWorkflowTime   time.Time                `json:"last_workflow_time"`
	StartedAt          time.Time                `json:"started_at"`
}

// NewOrchestratorMetrics membuat metrics baru
func NewOrchestratorMetrics() OrchestratorMetrics {
	return OrchestratorMetrics{
		StateDistribution: make(map[WorkflowState]int64),
		StartedAt:         time.Now(),
	}
}

// SuccessRate menghitung success rate
func (m *OrchestratorMetrics) SuccessRate() float64 {
	total := m.CompletedWorkflows + m.FailedWorkflows
	if total == 0 {
		return 100.0
	}
	return float64(m.CompletedWorkflows) / float64(total) * 100.0
}

// ============================================================
// WORKFLOW QUEUE
// ============================================================

// WorkflowQueue adalah priority queue untuk workflow
type WorkflowQueue struct {
	mu      sync.Mutex
	high    []WorkflowRequest
	normal  []WorkflowRequest
	low     []WorkflowRequest
	maxSize int
}

// NewWorkflowQueue membuat queue baru
func NewWorkflowQueue(maxSize int) *WorkflowQueue {
	return &WorkflowQueue{
		high:    make([]WorkflowRequest, 0),
		normal:  make([]WorkflowRequest, 0),
		low:     make([]WorkflowRequest, 0),
		maxSize: maxSize,
	}
}

// Enqueue menambah workflow ke queue
func (q *WorkflowQueue) Enqueue(req WorkflowRequest) error {
	q.mu.Lock()
	defer q.mu.Unlock()

	total := len(q.high) + len(q.normal) + len(q.low)
	if total >= q.maxSize {
		return fmt.Errorf("queue full: %d/%d", total, q.maxSize)
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

// Dequeue mengambil workflow berikutnya
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
	return nil, fmt.Errorf("queue empty")
}

// Size mengembalikan jumlah item
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
// DEFAULT ORCHESTRATOR (PARTIAL IMPLEMENTATION UNTUK TESTING)
// ============================================================

// DefaultOrchestrator adalah implementasi dasar Orchestrator
// untuk digunakan oleh implementor sebagai base atau untuk testing
type DefaultOrchestrator struct {
	callbacks *CallbackManager
	metrics   OrchestratorMetrics
	startTime time.Time
	healthy   bool
}

// NewDefaultOrchestrator membuat DefaultOrchestrator baru
func NewDefaultOrchestrator() *DefaultOrchestrator {
	return &DefaultOrchestrator{
		callbacks: NewCallbackManager(),
		metrics:   NewOrchestratorMetrics(),
		startTime: time.Now(),
		healthy:   true,
	}
}

// On mendaftarkan callback untuk event (SPEC FIX #4)
func (o *DefaultOrchestrator) On(event string, callback func(WorkflowStatus)) {
	o.callbacks.On(event, callback)
}

// RegisterCallback mendaftarkan callback (alias untuk On)
func (o *DefaultOrchestrator) RegisterCallback(event string, callback WorkflowCallback) {
	o.callbacks.On(event, callback)
}

// IsHealthy mengecek health
func (o *DefaultOrchestrator) IsHealthy() bool {
	return o.healthy
}

// GetUptime mendapatkan uptime
func (o *DefaultOrchestrator) GetUptime() time.Duration {
	return time.Since(o.startTime)
}

// GetWorkflowMetrics mendapatkan metrics
func (o *DefaultOrchestrator) GetWorkflowMetrics(ctx context.Context) (OrchestratorMetrics, error) {
	o.metrics.Uptime = time.Since(o.startTime)
	return o.metrics, nil
}

// Placeholder methods — implementor harus override
func (o *DefaultOrchestrator) StartWorkflow(ctx context.Context, req WorkflowRequest) (uuid.UUID, error) {
	return uuid.Nil, fmt.Errorf("not implemented")
}
func (o *DefaultOrchestrator) GetWorkflowStatus(ctx context.Context, id uuid.UUID) (WorkflowStatus, error) {
	return WorkflowStatus{}, fmt.Errorf("not implemented")
}
func (o *DefaultOrchestrator) PauseWorkflow(ctx context.Context, id uuid.UUID) error {
	return fmt.Errorf("not implemented")
}
func (o *DefaultOrchestrator) ResumeWorkflow(ctx context.Context, id uuid.UUID) error {
	return fmt.Errorf("not implemented")
}
func (o *DefaultOrchestrator) CancelWorkflow(ctx context.Context, id uuid.UUID) error {
	return fmt.Errorf("not implemented")
}
func (o *DefaultOrchestrator) GetWorkflowHistory(ctx context.Context, id uuid.UUID) ([]WorkflowStatus, error) {
	return nil, fmt.Errorf("not implemented")
}
func (o *DefaultOrchestrator) ListActiveWorkflows(ctx context.Context) ([]uuid.UUID, error) {
	return nil, fmt.Errorf("not implemented")
}
func (o *DefaultOrchestrator) ListAllWorkflows(ctx context.Context, limit, offset int) ([]WorkflowStatus, error) {
	return nil, fmt.Errorf("not implemented")
}
func (o *DefaultOrchestrator) GetWorkflowCount(ctx context.Context) (int, error) {
	return 0, fmt.Errorf("not implemented")
}
func (o *DefaultOrchestrator) Shutdown(ctx context.Context) error {
	o.healthy = false
	return nil
}

// ============================================================
// UNIT TESTS
// ============================================================

// Untuk menjalankan: go test ./shared/interfaces/
