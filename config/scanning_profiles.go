// config/scanning_profiles.go
// IWS v1.0 - Scanning Profiles Configuration
// Mendefinisikan dan mengelola berbagai profil scanning

package config

import (
	"fmt"
	"sync"
)

// ============================================================
// SCANNING PROFILE
// ============================================================

// ScanProfile mendefinisikan parameter untuk scanning
type ScanProfile struct {
	Name            string `json:"name"`
	Threads         int    `json:"threads"`
	TimeoutSecs     int    `json:"timeout_secs"`
	DelayMs         int    `json:"delay_ms"`
	MaxPages        int    `json:"max_pages"`
	MaxDepth        int    `json:"max_depth"`
	FollowRedirects bool   `json:"follow_redirects"`
	RespectRobots   bool   `json:"respect_robots"`
	EnableJS        bool   `json:"enable_js"`
	EnableStealth   bool   `json:"enable_stealth"`
}

// Validate memvalidasi profile
func (p *ScanProfile) Validate() error {
	if p.Threads < 1 || p.Threads > 200 {
		return fmt.Errorf("threads must be 1-200, got %d", p.Threads)
	}
	if p.TimeoutSecs < 1 || p.TimeoutSecs > 120 {
		return fmt.Errorf("timeout_secs must be 1-120, got %d", p.TimeoutSecs)
	}
	if p.DelayMs < 0 || p.DelayMs > 10000 {
		return fmt.Errorf("delay_ms must be 0-10000, got %d", p.DelayMs)
	}
	if p.MaxPages < 1 || p.MaxPages > 10000 {
		return fmt.Errorf("max_pages must be 1-10000, got %d", p.MaxPages)
	}
	if p.MaxDepth < 1 || p.MaxDepth > 10 {
		return fmt.Errorf("max_depth must be 1-10, got %d", p.MaxDepth)
	}
	return nil
}

// Predefined profiles
func ProfileModerate() ScanProfile {
	return ScanProfile{
		Name:            "moderate",
		Threads:         50,
		TimeoutSecs:     15,
		DelayMs:         100,
		MaxPages:        500,
		MaxDepth:        3,
		FollowRedirects: true,
		RespectRobots:   true,
		EnableJS:        false,
		EnableStealth:   false,
	}
}

func ProfileAggressive() ScanProfile {
	return ScanProfile{
		Name:            "aggressive",
		Threads:         100,
		TimeoutSecs:     10,
		DelayMs:         0,
		MaxPages:        1000,
		MaxDepth:        5,
		FollowRedirects: true,
		RespectRobots:   false,
		EnableJS:        false,
		EnableStealth:   false,
	}
}

func ProfileStealth() ScanProfile {
	return ScanProfile{
		Name:            "stealth",
		Threads:         10,
		TimeoutSecs:     30,
		DelayMs:         1000,
		MaxPages:        100,
		MaxDepth:        2,
		FollowRedirects: false,
		RespectRobots:   true,
		EnableJS:        false,
		EnableStealth:   true,
	}
}

func ProfileComprehensive() ScanProfile {
	return ScanProfile{
		Name:            "comprehensive",
		Threads:         30,
		TimeoutSecs:     20,
		DelayMs:         200,
		MaxPages:        2000,
		MaxDepth:        5,
		FollowRedirects: true,
		RespectRobots:   false,
		EnableJS:        true,
		EnableStealth:   false,
	}
}

// ============================================================
// SCANNING PROFILES MANAGER
// ============================================================

// ScanningProfiles mengelola semua profil scanning
type ScanningProfiles struct {
	mu       sync.RWMutex
	profiles map[string]ScanProfile
	defaultProfile string
}

// NewScanningProfiles membuat manager dengan predefined profiles
func NewScanningProfiles() *ScanningProfiles {
	sp := &ScanningProfiles{
		profiles: make(map[string]ScanProfile),
		defaultProfile: "moderate",
	}

	// Register predefined profiles
	sp.Register(ProfileModerate())
	sp.Register(ProfileAggressive())
	sp.Register(ProfileStealth())
	sp.Register(ProfileComprehensive())

	return sp
}

// Register menambah profil baru
func (sp *ScanningProfiles) Register(profile ScanProfile) error {
	sp.mu.Lock()
	defer sp.mu.Unlock()

	if err := profile.Validate(); err != nil {
		return fmt.Errorf("invalid profile '%s': %w", profile.Name, err)
	}

	if _, exists := sp.profiles[profile.Name]; exists {
		return fmt.Errorf("profile '%s' already exists", profile.Name)
	}

	sp.profiles[profile.Name] = profile
	return nil
}

// Unregister menghapus profil
func (sp *ScanningProfiles) Unregister(name string) error {
	sp.mu.Lock()
	defer sp.mu.Unlock()

	if name == sp.defaultProfile {
		return fmt.Errorf("cannot unregister default profile '%s'", name)
	}

	if _, exists := sp.profiles[name]; !exists {
		return fmt.Errorf("profile '%s' not found", name)
	}

	delete(sp.profiles, name)
	return nil
}

// GetProfile mendapatkan profil berdasarkan nama
func (sp *ScanningProfiles) GetProfile(name string) (ScanProfile, error) {
	sp.mu.RLock()
	defer sp.mu.RUnlock()

	profile, exists := sp.profiles[name]
	if !exists {
		return ScanProfile{}, fmt.Errorf("profile '%s' not found", name)
	}

	return profile, nil
}

// GetDefaultProfile mendapatkan profil default
func (sp *ScanningProfiles) GetDefaultProfile() ScanProfile {
	sp.mu.RLock()
	defer sp.mu.RUnlock()

	profile, _ := sp.profiles[sp.defaultProfile]
	return profile
}

// SetDefaultProfile mengatur profil default
func (sp *ScanningProfiles) SetDefaultProfile(name string) error {
	sp.mu.Lock()
	defer sp.mu.Unlock()

	if _, exists := sp.profiles[name]; !exists {
		return fmt.Errorf("profile '%s' not found", name)
	}

	sp.defaultProfile = name
	return nil
}

// ListProfiles mengembalikan semua nama profil
func (sp *ScanningProfiles) ListProfiles() []string {
	sp.mu.RLock()
	defer sp.mu.RUnlock()

	names := make([]string, 0, len(sp.profiles))
	for name := range sp.profiles {
		names = append(names, name)
	}
	return names
}

// Count mengembalikan jumlah profil
func (sp *ScanningProfiles) Count() int {
	sp.mu.RLock()
	defer sp.mu.RUnlock()
	return len(sp.profiles)
}

// GetProfileRecommendation memberikan rekomendasi profil berdasarkan target
func (sp *ScanningProfiles) GetProfileRecommendation(targetType string) string {
	switch targetType {
	case "banking", "financial", "government", "healthcare":
		return "stealth"
	case "testing", "development", "staging", "internal":
		return "aggressive"
	case "compliance", "audit", "pentest":
		return "comprehensive"
	default:
		return "moderate"
	}
}

// CreateCustomProfile membuat profil kustom
func (sp *ScanningProfiles) CreateCustomProfile(name string, threads, timeout, delay, maxPages, maxDepth int, followRedirects, respectRobots, enableJS, enableStealth bool) error {
	profile := ScanProfile{
		Name:            name,
		Threads:         threads,
		TimeoutSecs:     timeout,
		DelayMs:         delay,
		MaxPages:        maxPages,
		MaxDepth:        maxDepth,
		FollowRedirects: followRedirects,
		RespectRobots:   respectRobots,
		EnableJS:        enableJS,
		EnableStealth:   enableStealth,
	}

	return sp.Register(profile)
}

// UpdateProfile memperbarui profil yang sudah ada
func (sp *ScanningProfiles) UpdateProfile(name string, profile ScanProfile) error {
	sp.mu.Lock()
	defer sp.mu.Unlock()

	if _, exists := sp.profiles[name]; !exists {
		return fmt.Errorf("profile '%s' not found", name)
	}

	profile.Name = name
	if err := profile.Validate(); err != nil {
		return err
	}

	sp.profiles[name] = profile
	return nil
}

// GetProfilesByType mengembalikan profil berdasarkan karakteristik
func (sp *ScanningProfiles) GetProfilesByType(stealth bool) []ScanProfile {
	sp.mu.RLock()
	defer sp.mu.RUnlock()

	var result []ScanProfile
	for _, p := range sp.profiles {
		if p.EnableStealth == stealth {
			result = append(result, p)
		}
	}
	return result
}

// CloneProfile mengkloning profil
func (sp *ScanningProfiles) CloneProfile(sourceName, newName string) error {
	sp.mu.Lock()
	defer sp.mu.Unlock()

	source, exists := sp.profiles[sourceName]
	if !exists {
		return fmt.Errorf("source profile '%s' not found", sourceName)
	}

	if _, exists := sp.profiles[newName]; exists {
		return fmt.Errorf("target profile '%s' already exists", newName)
	}

	clone := source
	clone.Name = newName
	sp.profiles[newName] = clone
	return nil
}

// ============================================================
// PROFILE COMPARISON
// ============================================================

// CompareProfiles membandingkan dua profil
func CompareProfiles(a, b ScanProfile) []string {
	var diffs []string

	if a.Threads != b.Threads {
		diffs = append(diffs, fmt.Sprintf("Threads: %d -> %d", a.Threads, b.Threads))
	}
	if a.TimeoutSecs != b.TimeoutSecs {
		diffs = append(diffs, fmt.Sprintf("Timeout: %ds -> %ds", a.TimeoutSecs, b.TimeoutSecs))
	}
	if a.DelayMs != b.DelayMs {
		diffs = append(diffs, fmt.Sprintf("Delay: %dms -> %dms", a.DelayMs, b.DelayMs))
	}
	if a.MaxPages != b.MaxPages {
		diffs = append(diffs, fmt.Sprintf("MaxPages: %d -> %d", a.MaxPages, b.MaxPages))
	}
	if a.MaxDepth != b.MaxDepth {
		diffs = append(diffs, fmt.Sprintf("MaxDepth: %d -> %d", a.MaxDepth, b.MaxDepth))
	}
	if a.FollowRedirects != b.FollowRedirects {
		diffs = append(diffs, fmt.Sprintf("FollowRedirects: %v -> %v", a.FollowRedirects, b.FollowRedirects))
	}
	if a.RespectRobots != b.RespectRobots {
		diffs = append(diffs, fmt.Sprintf("RespectRobots: %v -> %v", a.RespectRobots, b.RespectRobots))
	}
	if a.EnableJS != b.EnableJS {
		diffs = append(diffs, fmt.Sprintf("EnableJS: %v -> %v", a.EnableJS, b.EnableJS))
	}
	if a.EnableStealth != b.EnableStealth {
		diffs = append(diffs, fmt.Sprintf("EnableStealth: %v -> %v", a.EnableStealth, b.EnableStealth))
	}

	return diffs
}

// ProfileSummary mengembalikan ringkasan profil
func (p *ScanProfile) Summary() string {
	return fmt.Sprintf(
		"%s: threads=%d, timeout=%ds, delay=%dms, pages=%d, depth=%d, redirects=%v, robots=%v, js=%v, stealth=%v",
		p.Name, p.Threads, p.TimeoutSecs, p.DelayMs, p.MaxPages, p.MaxDepth,
		p.FollowRedirects, p.RespectRobots, p.EnableJS, p.EnableStealth,
	)
}

// IsAggressive mengecek apakah profil agresif
func (p *ScanProfile) IsAggressive() bool {
	return p.Threads >= 80 && p.DelayMs <= 10
}

// IsStealth mengecek apakah profil stealth
func (p *ScanProfile) IsStealth() bool {
	return p.EnableStealth || (p.Threads <= 15 && p.DelayMs >= 500)
}

// EstimateDuration mengestimasi durasi scan dalam detik
func (p *ScanProfile) EstimateDuration() int {
	baseTime := p.MaxPages * (p.TimeoutSecs + p.DelayMs/1000)
	parallelFactor := baseTime / p.Threads
	return parallelFactor + (p.TimeoutSecs * 2)
}

// ============================================================
// UNIT TESTS
// ============================================================

// Untuk menjalankan: go test ./config/
