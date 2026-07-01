// agents/reconnaissance_agent.go
// IWS v1.0 - Reconnaissance Agent
// Melakukan pengintaian awal terhadap target website

package agents

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/google/uuid"
)

// ReconResult menyimpan hasil reconnaissance
type ReconResult struct {
	Domain       string              `json:"domain"`
	Whois        WhoisInfo           `json:"whois"`
	DNS          DnsInfo             `json:"dns"`
	Subdomains   []string            `json:"subdomains"`
	Technologies []string            `json:"technologies"`
	CreatedAt    time.Time           `json:"created_at"`
	CacheKey     string              `json:"cache_key"`
	FromCache    bool                `json:"from_cache"`
}

// WhoisInfo menyimpan data WHOIS
type WhoisInfo struct {
	Registrar     string    `json:"registrar"`
	CreationDate  time.Time `json:"creation_date"`
	ExpiryDate    time.Time `json:"expiry_date"`
	Nameservers   []string  `json:"nameservers"`
	RegistrantOrg string    `json:"registrant_org"`
	Country       string    `json:"country"`
	IsRedacted    bool      `json:"is_redacted"`
}

// DnsInfo menyimpan DNS records
type DnsInfo struct {
	ARecords     []string `json:"a_records"`
	AAAARecords  []string `json:"aaaa_records"`
	CNAME         []string `json:"cname"`
	MXRecords    []string `json:"mx_records"`
	TXTRecords   []string `json:"txt_records"`
	NSRecords    []string `json:"ns_records"`
	SOA           string   `json:"soa"`
	DNSSEC        bool     `json:"dnssec"`
}

// ReconAgent melakukan reconnaissance
type ReconAgent struct {
	id       uuid.UUID
	name     string
	state    string
	cache    map[string]*ReconResult
	cacheTTL time.Duration
	mu       sync.RWMutex
}

// NewReconAgent membuat agent baru
func NewReconAgent() *ReconAgent {
	return &ReconAgent{
		id:       uuid.New(),
		name:     "reconnaissance-agent",
		state:    "uninitialized",
		cache:    make(map[string]*ReconResult),
		cacheTTL: 24 * time.Hour,
	}
}

// ID returns agent ID
func (a *ReconAgent) ID() uuid.UUID { return a.id }

// Name returns agent name
func (a *ReconAgent) Name() string { return a.name }

// State returns current state
func (a *ReconAgent) State() string { return a.state }

// SetState updates state
func (a *ReconAgent) SetState(s string) { a.state = s }

// Reconnaissance melakukan recon terhadap domain
func (a *ReconAgent) Reconnaissance(ctx context.Context, domain string) (*ReconResult, error) {
	// Cek cache
	a.mu.RLock()
	if cached, ok := a.cache[domain]; ok {
		if time.Since(cached.CreatedAt) < a.cacheTTL {
			a.mu.RUnlock()
			return cached, nil
		}
	}
	a.mu.RUnlock()

	result := &ReconResult{
		Domain:    domain,
		CreatedAt: time.Now(),
		CacheKey:  fmt.Sprintf("recon:%s:%d", domain, time.Now().Unix()),
	}

	var wg sync.WaitGroup
	var mu sync.Mutex
	errors := make([]error, 0)

	// Stage 1: WHOIS
	wg.Add(1)
	go func() {
		defer wg.Done()
		whois, err := a.gatherWhois(domain)
		mu.Lock()
		if err != nil {
			errors = append(errors, err)
		} else {
			result.Whois = *whois
		}
		mu.Unlock()
	}()

	// Stage 2: DNS
	wg.Add(1)
	go func() {
		defer wg.Done()
		dns, err := a.gatherDNS(domain)
		mu.Lock()
		if err != nil {
			errors = append(errors, err)
		} else {
			result.DNS = *dns
		}
		mu.Unlock()
	}()

	// Stage 3: Subdomains
	wg.Add(1)
	go func() {
		defer wg.Done()
		subs, err := a.gatherSubdomains(domain)
		mu.Lock()
		if err != nil {
			errors = append(errors, err)
		} else {
			result.Subdomains = subs
		}
		mu.Unlock()
	}()

	// Stage 4: Technologies
	wg.Add(1)
	go func() {
		defer wg.Done()
		techs, err := a.gatherTechnologies(domain)
		mu.Lock()
		if err != nil {
			errors = append(errors, err)
		} else {
			result.Technologies = techs
		}
		mu.Unlock()
	}()

	wg.Wait()

	if len(errors) > 0 && result.Whois.Registrar == "" {
		return nil, fmt.Errorf("reconnaissance failed: %v", errors)
	}

	// Simpan ke cache
	a.mu.Lock()
	a.cache[domain] = result
	a.mu.Unlock()

	return result, nil
}

func (a *ReconAgent) gatherWhois(domain string) (*WhoisInfo, error) {
	// Production: gunakan whois library
	return &WhoisInfo{
		Registrar:    "Example Registrar",
		CreationDate: time.Now().AddDate(-2, 0, 0),
		ExpiryDate:   time.Now().AddDate(1, 0, 0),
		Nameservers:  []string{"ns1.example.com", "ns2.example.com"},
		Country:      "US",
	}, nil
}

func (a *ReconAgent) gatherDNS(domain string) (*DnsInfo, error) {
	// Production: gunakan net.LookupHost, net.LookupMX, dll
	return &DnsInfo{
		ARecords:    []string{"93.184.216.34"},
		AAAARecords: []string{"2606:2800:220:1:248:1893:25c8:1946"},
		MXRecords:   []string{"mail.example.com"},
		NSRecords:   []string{"ns1.example.com"},
		DNSSEC:      true,
	}, nil
}

func (a *ReconAgent) gatherSubdomains(domain string) ([]string, error) {
	// Production: brute force + certificate transparency
	commonSubs := []string{"www", "mail", "ftp", "admin", "api", "dev", "staging", "blog", "shop"}
	results := make([]string, 0)
	for _, sub := range commonSubs {
		results = append(results, fmt.Sprintf("%s.%s", sub, domain))
	}
	return results, nil
}

func (a *ReconAgent) gatherTechnologies(domain string) ([]string, error) {
	// Production: Wappalyzer patterns
	return []string{"nginx", "React", "jQuery", "Cloudflare"}, nil
}

// GetCacheStats returns cache statistics
func (a *ReconAgent) GetCacheStats() map[string]interface{} {
	a.mu.RLock()
	defer a.mu.RUnlock()
	return map[string]interface{}{
		"size":  len(a.cache),
		"ttl":   a.cacheTTL.String(),
		"keys":  func() []string { keys := make([]string, 0, len(a.cache)); for k := range a.cache { keys = append(keys, k) }; return keys }(),
	}
}

// ClearCache clears the reconnaissance cache
func (a *ReconAgent) ClearCache() {
	a.mu.Lock()
	defer a.mu.Unlock()
	a.cache = make(map[string]*ReconResult)
}
