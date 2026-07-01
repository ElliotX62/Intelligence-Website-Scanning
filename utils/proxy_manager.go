// utils/proxy_manager.go
// IWS v1.0 - Proxy Manager
// Mengelola proxy untuk stealth scanning dengan rotation, validation, dan anonymity tracking

package utils

import (
	"fmt"
	"net/http"
	"net/url"
	"sync"
	"sync/atomic"
	"time"
)

// ============================================================
// PROXY TYPES
// ============================================================

// ProxyType mendefinisikan tipe proxy
type ProxyType string

const (
	ProxyTypeHTTP   ProxyType = "http"
	ProxyTypeHTTPS  ProxyType = "https"
	ProxyTypeSOCKS5 ProxyType = "socks5"
)

// AnonymityLevel mendefinisikan tingkat anonimitas proxy
type AnonymityLevel int

const (
	AnonymityTransparent AnonymityLevel = iota // Mengirim IP asli
	AnonymityAnonymous                          // Menyembunyikan IP, tapi diketahui sebagai proxy
	AnonymityElite                              // Tidak ada jejak proxy
	AnonymityUnknown
)

func (a AnonymityLevel) String() string {
	switch a {
	case AnonymityTransparent:
		return "transparent"
	case AnonymityAnonymous:
		return "anonymous"
	case AnonymityElite:
		return "elite"
	default:
		return "unknown"
	}
}

// ============================================================
// PROXY STRUCT
// ============================================================

// Proxy merepresentasikan satu proxy server
type Proxy struct {
	URL             string          `json:"url"`
	Type            ProxyType       `json:"type"`
	Anonymity       AnonymityLevel  `json:"anonymity"`
	Country         string          `json:"country"`
	LastValidated   time.Time       `json:"last_validated"`
	FailCount       int             `json:"fail_count"`
	SuccessCount    int64           `json:"success_count"`
	AvgResponseMs   int64           `json:"avg_response_ms"`
	IsActive        bool            `json:"is_active"`
	MaxFails        int             `json:"max_fails"`
	parsedURL       *url.URL
}

// Validate memvalidasi proxy dengan test connection
func (p *Proxy) Validate(timeout time.Duration) bool {
	proxyURL, err := url.Parse(p.URL)
	if err != nil {
		return false
	}
	p.parsedURL = proxyURL

	transport := &http.Transport{
		Proxy: http.ProxyURL(proxyURL),
	}

	client := &http.Client{
		Transport: transport,
		Timeout:   timeout,
	}

	start := time.Now()
	resp, err := client.Get("https://httpbin.org/ip")
	if err != nil {
		p.FailCount++
		if p.FailCount >= p.MaxFails {
			p.IsActive = false
		}
		return false
	}
	defer resp.Body.Close()

	elapsed := time.Since(start)
	p.LastValidated = time.Now()
	p.FailCount = 0
	p.IsActive = true

	// Update average response time
	oldCount := atomic.LoadInt64(&p.SuccessCount)
	newCount := oldCount + 1
	atomic.StoreInt64(&p.SuccessCount, newCount)

	oldAvg := atomic.LoadInt64(&p.AvgResponseMs)
	newAvg := (oldAvg*oldCount + elapsed.Milliseconds()) / newCount
	atomic.StoreInt64(&p.AvgResponseMs, newAvg)

	return true
}

// GetResponseTime mendapatkan rata-rata response time
func (p *Proxy) GetResponseTime() time.Duration {
	return time.Duration(atomic.LoadInt64(&p.AvgResponseMs)) * time.Millisecond
}

// GetSuccessCount mendapatkan jumlah sukses
func (p *Proxy) GetSuccessCount() int64 {
	return atomic.LoadInt64(&p.SuccessCount)
}

// MarkFailed menandai proxy gagal
func (p *Proxy) MarkFailed() {
	p.FailCount++
	if p.FailCount >= p.MaxFails {
		p.IsActive = false
	}
}

// MarkSuccess menandai proxy sukses
func (p *Proxy) MarkSuccess() {
	p.FailCount = 0
	atomic.AddInt64(&p.SuccessCount, 1)
}

// ============================================================
// PROXY MANAGER
// ============================================================

// ProxyManager mengelola pool proxy
type ProxyManager struct {
	mu            sync.RWMutex
	proxies       []*Proxy
	currentIndex  uint64
	validateTimer *time.Ticker
	stopChan      chan struct{}
}

// NewProxyManager membuat ProxyManager baru
func NewProxyManager() *ProxyManager {
	return &ProxyManager{
		proxies:      make([]*Proxy, 0),
		currentIndex: 0,
		stopChan:     make(chan struct{}),
	}
}

// AddProxy menambah proxy ke pool
func (pm *ProxyManager) AddProxy(proxyURL string, proxyType ProxyType) *Proxy {
	pm.mu.Lock()
	defer pm.mu.Unlock()

	proxy := &Proxy{
		URL:       proxyURL,
		Type:      proxyType,
		Anonymity: AnonymityUnknown,
		IsActive:  true,
		MaxFails:  3,
	}

	pm.proxies = append(pm.proxies, proxy)
	return proxy
}

// RemoveProxy menghapus proxy dari pool
func (pm *ProxyManager) RemoveProxy(proxyURL string) bool {
	pm.mu.Lock()
	defer pm.mu.Unlock()

	for i, p := range pm.proxies {
		if p.URL == proxyURL {
			pm.proxies = append(pm.proxies[:i], pm.proxies[i+1:]...)
			return true
		}
	}
	return false
}

// GetProxy mendapatkan proxy dengan round-robin rotation
func (pm *ProxyManager) GetProxy() *Proxy {
	pm.mu.RLock()
	defer pm.mu.RUnlock()

	if len(pm.proxies) == 0 {
		return nil
	}

	activeProxies := make([]*Proxy, 0)
	for _, p := range pm.proxies {
		if p.IsActive {
			activeProxies = append(activeProxies, p)
		}
	}

	if len(activeProxies) == 0 {
		return nil
	}

	idx := atomic.AddUint64(&pm.currentIndex, 1) % uint64(len(activeProxies))
	return activeProxies[idx]
}

// GetRandomProxy mendapatkan proxy random
func (pm *ProxyManager) GetRandomProxy() *Proxy {
	pm.mu.RLock()
	defer pm.mu.RUnlock()

	activeProxies := make([]*Proxy, 0)
	for _, p := range pm.proxies {
		if p.IsActive {
			activeProxies = append(activeProxies, p)
		}
	}

	if len(activeProxies) == 0 {
		return nil
	}

	// Simple pseudo-random based on time
	idx := time.Now().UnixNano() % int64(len(activeProxies))
	if idx < 0 {
		idx = -idx
	}
	return activeProxies[idx]
}

// GetProxyByCountry mendapatkan proxy dari negara tertentu
func (pm *ProxyManager) GetProxyByCountry(country string) *Proxy {
	pm.mu.RLock()
	defer pm.mu.RUnlock()

	for _, p := range pm.proxies {
		if p.IsActive && p.Country == country {
			return p
		}
	}
	return nil
}

// GetProxyByType mendapatkan proxy dengan tipe tertentu
func (pm *ProxyManager) GetProxyByType(proxyType ProxyType) *Proxy {
	pm.mu.RLock()
	defer pm.mu.RUnlock()

	for _, p := range pm.proxies {
		if p.IsActive && p.Type == proxyType {
			return p
		}
	}
	return nil
}

// GetEliteProxy mendapatkan proxy elite (anonimitas tertinggi)
func (pm *ProxyManager) GetEliteProxy() *Proxy {
	pm.mu.RLock()
	defer pm.mu.RUnlock()

	for _, p := range pm.proxies {
		if p.IsActive && p.Anonymity == AnonymityElite {
			return p
		}
	}
	return pm.GetProxy() // Fallback ke round-robin
}

// ValidateAll memvalidasi semua proxy
func (pm *ProxyManager) ValidateAll(timeout time.Duration) {
	pm.mu.RLock()
	proxies := make([]*Proxy, len(pm.proxies))
	copy(proxies, pm.proxies)
	pm.mu.RUnlock()

	var wg sync.WaitGroup
	for _, p := range proxies {
		wg.Add(1)
		go func(proxy *Proxy) {
			defer wg.Done()
			proxy.Validate(timeout)
		}(p)
	}
	wg.Wait()
}

// StartAutoValidation memulai validasi otomatis setiap interval
func (pm *ProxyManager) StartAutoValidation(interval time.Duration, timeout time.Duration) {
	pm.validateTimer = time.NewTicker(interval)
	go func() {
		for {
			select {
			case <-pm.validateTimer.C:
				pm.ValidateAll(timeout)
				pm.RemoveDeadProxies()
			case <-pm.stopChan:
				pm.validateTimer.Stop()
				return
			}
		}
	}()
}

// StopAutoValidation menghentikan validasi otomatis
func (pm *ProxyManager) StopAutoValidation() {
	close(pm.stopChan)
}

// RemoveDeadProxies menghapus proxy yang tidak aktif
func (pm *ProxyManager) RemoveDeadProxies() int {
	pm.mu.Lock()
	defer pm.mu.Unlock()

	active := make([]*Proxy, 0)
	removed := 0
	for _, p := range pm.proxies {
		if p.IsActive {
			active = append(active, p)
		} else {
			removed++
		}
	}
	pm.proxies = active
	return removed
}

// Count mengembalikan jumlah proxy
func (pm *ProxyManager) Count() int {
	pm.mu.RLock()
	defer pm.mu.RUnlock()
	return len(pm.proxies)
}

// ActiveCount mengembalikan jumlah proxy aktif
func (pm *ProxyManager) ActiveCount() int {
	pm.mu.RLock()
	defer pm.mu.RUnlock()
	count := 0
	for _, p := range pm.proxies {
		if p.IsActive {
			count++
		}
	}
	return count
}

// ListProxies mengembalikan semua proxy
func (pm *ProxyManager) ListProxies() []*Proxy {
	pm.mu.RLock()
	defer pm.mu.RUnlock()

	result := make([]*Proxy, len(pm.proxies))
	copy(result, pm.proxies)
	return result
}

// GetStats mengembalikan statistik proxy pool
func (pm *ProxyManager) GetStats() map[string]interface{} {
	pm.mu.RLock()
	defer pm.mu.RUnlock()

	total := len(pm.proxies)
	active := 0
	elite := 0
	var totalSuccess int64
	var totalAvgMs int64

	for _, p := range pm.proxies {
		if p.IsActive {
			active++
		}
		if p.Anonymity == AnonymityElite {
			elite++
		}
		totalSuccess += atomic.LoadInt64(&p.SuccessCount)
	}

	if active > 0 {
		totalAvgMs = totalAvgMs / int64(active)
	}

	return map[string]interface{}{
		"total":     total,
		"active":    active,
		"elite":     elite,
		"successes": totalSuccess,
	}
}

// ImportFromList mengimport proxy dari list string
func (pm *ProxyManager) ImportFromList(proxyList []string, proxyType ProxyType) int {
	count := 0
	for _, p := range proxyList {
		if p != "" {
			pm.AddProxy(p, proxyType)
			count++
		}
	}
	return count
}

// DetectAnonymity mendeteksi tingkat anonimitas proxy
func (pm *ProxyManager) DetectAnonymity(proxy *Proxy, timeout time.Duration) AnonymityLevel {
	if proxy.parsedURL == nil {
		var err error
		proxy.parsedURL, err = url.Parse(proxy.URL)
		if err != nil {
			return AnonymityUnknown
		}
	}

	transport := &http.Transport{
		Proxy: http.ProxyURL(proxy.parsedURL),
	}

	client := &http.Client{
		Transport: transport,
		Timeout:   timeout,
	}

	resp, err := client.Get("https://httpbin.org/headers")
	if err != nil {
		return AnonymityUnknown
	}
	defer resp.Body.Close()

	// Cek headers yang mengindikasikan proxy
	hasProxyHeaders := false
	proxyHeaders := []string{
		"X-Forwarded-For", "X-Real-IP", "Via",
		"Proxy-Connection", "X-Proxy-ID",
	}

	for _, header := range proxyHeaders {
		if resp.Header.Get(header) != "" {
			hasProxyHeaders = true
			break
		}
	}

	if hasProxyHeaders {
		// Cek apakah IP asli terekspos
		if resp.Header.Get("X-Forwarded-For") != "" {
			return AnonymityTransparent
		}
		return AnonymityAnonymous
	}

	return AnonymityElite
}

// ============================================================
// PROXY ROTATION STRATEGIES
// ============================================================

// RoundRobinStrategy — rotasi berurutan
func RoundRobinStrategy(proxies []*Proxy, currentIndex *uint64) *Proxy {
	if len(proxies) == 0 {
		return nil
	}
	idx := atomic.AddUint64(currentIndex, 1) % uint64(len(proxies))
	return proxies[idx]
}

// RandomStrategy — rotasi random
func RandomStrategy(proxies []*Proxy) *Proxy {
	if len(proxies) == 0 {
		return nil
	}
	idx := time.Now().UnixNano() % int64(len(proxies))
	if idx < 0 {
		idx = -idx
	}
	return proxies[idx]
}

// BestPerformanceStrategy — pilih proxy dengan response time terbaik
func BestPerformanceStrategy(proxies []*Proxy) *Proxy {
	if len(proxies) == 0 {
		return nil
	}

	var best *Proxy
	var bestTime int64 = int64(^uint64(0) >> 1) // Max int64

	for _, p := range proxies {
		if p.IsActive {
			avgTime := atomic.LoadInt64(&p.AvgResponseMs)
			if avgTime < bestTime && avgTime > 0 {
				bestTime = avgTime
				best = p
			}
		}
	}

	if best == nil {
		// Fallback ke proxy pertama yang aktif
		for _, p := range proxies {
			if p.IsActive {
				return p
			}
		}
	}

	return best
}

// ============================================================
// PROXY TESTER
// ============================================================

// TestProxy menguji koneksi proxy
func TestProxy(proxyURL string, timeout time.Duration) error {
	parsedURL, err := url.Parse(proxyURL)
	if err != nil {
		return fmt.Errorf("invalid proxy URL: %w", err)
	}

	transport := &http.Transport{
		Proxy: http.ProxyURL(parsedURL),
	}

	client := &http.Client{
		Transport: transport,
		Timeout:   timeout,
	}

	resp, err := client.Get("https://httpbin.org/ip")
	if err != nil {
		return fmt.Errorf("proxy connection failed: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("proxy returned status %d", resp.StatusCode)
	}

	return nil
}

// ============================================================
// UNIT TESTS
// ============================================================

// Untuk menjalankan: go test ./utils/
