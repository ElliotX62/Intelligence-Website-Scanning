-- database/schema.sql
-- IWS v1.0 - Database Schema
-- Mendefinisikan struktur database lengkap

-- ============================================================
-- EXTENSIONS
-- ============================================================
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================================
-- USERS
-- ============================================================
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    role VARCHAR(20) DEFAULT 'user' CHECK (role IN ('admin', 'user', 'guest')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT true,
    api_key VARCHAR(64) UNIQUE,
    settings JSONB DEFAULT '{}'
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_api_key ON users(api_key);
CREATE INDEX idx_users_role ON users(role);

-- ============================================================
-- SCAN RESULTS
-- ============================================================
CREATE TABLE IF NOT EXISTS scan_results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    target_url VARCHAR(2048) NOT NULL,
    target_ip VARCHAR(45),
    scan_profile VARCHAR(50) DEFAULT 'moderate',
    started_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP WITH TIME ZONE,
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending','active','completed','failed','cancelled')),
    result JSONB,
    summary TEXT,
    risk_score DECIMAL(5,2),
    scan_metadata JSONB DEFAULT '{}'
);

CREATE INDEX idx_scan_results_user_id ON scan_results(user_id);
CREATE INDEX idx_scan_results_status ON scan_results(status);
CREATE INDEX idx_scan_results_started ON scan_results(started_at DESC);
CREATE INDEX idx_scan_results_target ON scan_results(target_url);
CREATE INDEX idx_scan_results_risk ON scan_results(risk_score);

-- ============================================================
-- VULNERABILITIES
-- ============================================================
CREATE TABLE IF NOT EXISTS vulnerabilities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    scan_id UUID REFERENCES scan_results(id) ON DELETE CASCADE,
    cve_id VARCHAR(50),
    title VARCHAR(500) NOT NULL,
    description TEXT,
    severity VARCHAR(10) CHECK (severity IN ('critical','high','medium','low','info')),
    cvss_score DECIMAL(3,1),
    cvss_vector VARCHAR(100),
    affected_component VARCHAR(500),
    affected_version VARCHAR(100),
    fixed_version VARCHAR(100),
    remediation TEXT,
    status VARCHAR(20) DEFAULT 'open' CHECK (status IN ('open','in_progress','fixed','wont_fix','false_positive','duplicate')),
    discovered_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    fixed_at TIMESTAMP WITH TIME ZONE,
    assigned_to UUID REFERENCES users(id)
);

CREATE INDEX idx_vulns_scan_id ON vulnerabilities(scan_id);
CREATE INDEX idx_vulns_severity ON vulnerabilities(severity);
CREATE INDEX idx_vulns_cve ON vulnerabilities(cve_id);
CREATE INDEX idx_vulns_status ON vulnerabilities(status);

-- ============================================================
-- MODULE RESULTS
-- ============================================================
CREATE TABLE IF NOT EXISTS module_results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    scan_id UUID REFERENCES scan_results(id) ON DELETE CASCADE,
    module_name VARCHAR(100) NOT NULL,
    module_type VARCHAR(50),
    status VARCHAR(20) DEFAULT 'pending',
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    duration_ms BIGINT,
    findings_count INTEGER DEFAULT 0,
    errors_count INTEGER DEFAULT 0,
    data JSONB DEFAULT '{}'
);

CREATE INDEX idx_module_results_scan ON module_results(scan_id);
CREATE INDEX idx_module_results_name ON module_results(module_name);

-- ============================================================
-- FINDINGS
-- ============================================================
CREATE TABLE IF NOT EXISTS findings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    scan_id UUID REFERENCES scan_results(id) ON DELETE CASCADE,
    module_name VARCHAR(100),
    finding_type VARCHAR(100),
    title VARCHAR(500) NOT NULL,
    description TEXT,
    severity VARCHAR(10),
    confidence DECIMAL(3,2) DEFAULT 0.5,
    location TEXT,
    evidence JSONB,
    remediation TEXT,
    discovered_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_findings_scan ON findings(scan_id);
CREATE INDEX idx_findings_severity ON findings(severity);
CREATE INDEX idx_findings_type ON findings(finding_type);

-- ============================================================
-- ANALYSIS RESULTS
-- ============================================================
CREATE TABLE IF NOT EXISTS analysis_results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    scan_id UUID REFERENCES scan_results(id) ON DELETE CASCADE,
    started_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP WITH TIME ZONE,
    status VARCHAR(20) DEFAULT 'pending',
    risk_score DECIMAL(5,2),
    findings_count INTEGER DEFAULT 0,
    critical_count INTEGER DEFAULT 0,
    high_count INTEGER DEFAULT 0,
    medium_count INTEGER DEFAULT 0,
    low_count INTEGER DEFAULT 0,
    summary TEXT,
    result JSONB DEFAULT '{}'
);

CREATE INDEX idx_analysis_scan ON analysis_results(scan_id);

-- ============================================================
-- AGENT STATES
-- ============================================================
CREATE TABLE IF NOT EXISTS agent_states (
    id SERIAL PRIMARY KEY,
    agent_id UUID NOT NULL,
    agent_name VARCHAR(100) NOT NULL,
    agent_type VARCHAR(50),
    state VARCHAR(20) NOT NULL DEFAULT 'uninitialized',
    last_heartbeat TIMESTAMP WITH TIME ZONE,
    state_data JSONB NOT NULL DEFAULT '{}',
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_agent_states_agent ON agent_states(agent_id);
CREATE INDEX idx_agent_states_name ON agent_states(agent_name);
CREATE INDEX idx_agent_states_heartbeat ON agent_states(last_heartbeat);

-- ============================================================
-- REPORTS
-- ============================================================
CREATE TABLE IF NOT EXISTS reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    scan_id UUID REFERENCES scan_results(id) ON DELETE CASCADE,
    report_type VARCHAR(50) DEFAULT 'full',
    format VARCHAR(10) DEFAULT 'json',
    file_path TEXT,
    size_bytes BIGINT DEFAULT 0,
    generated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    generated_by VARCHAR(100),
    metadata JSONB DEFAULT '{}'
);

CREATE INDEX idx_reports_scan ON reports(scan_id);
CREATE INDEX idx_reports_type ON reports(report_type);

-- ============================================================
-- CONFIGURATION
-- ============================================================
CREATE TABLE IF NOT EXISTS configuration (
    id SERIAL PRIMARY KEY,
    key VARCHAR(255) UNIQUE NOT NULL,
    value JSONB NOT NULL,
    description TEXT,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_by VARCHAR(100)
);

CREATE INDEX idx_config_key ON configuration(key);

-- ============================================================
-- API KEYS
-- ============================================================
CREATE TABLE IF NOT EXISTS api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    api_key VARCHAR(64) UNIQUE NOT NULL,
    permissions JSONB DEFAULT '[]',
    rate_limit INTEGER DEFAULT 1000,
    expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT true
);

CREATE INDEX idx_api_keys_key ON api_keys(api_key);
CREATE INDEX idx_api_keys_user ON api_keys(user_id);

-- ============================================================
-- MIGRATIONS TRACKING
-- ============================================================
CREATE TABLE IF NOT EXISTS migrations (
    id SERIAL PRIMARY KEY,
    migration_name VARCHAR(255) UNIQUE NOT NULL,
    executed_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    checksum VARCHAR(64)
);

-- ============================================================
-- AUDIT LOG
-- ============================================================
CREATE TABLE IF NOT EXISTS audit_log (
    id SERIAL PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(50),
    resource_id VARCHAR(100),
    details JSONB DEFAULT '{}',
    ip_address VARCHAR(45),
    user_agent TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_audit_user ON audit_log(user_id);
CREATE INDEX idx_audit_action ON audit_log(action);
CREATE INDEX idx_audit_created ON audit_log(created_at DESC);

-- ============================================================
-- DEFAULT DATA
-- ============================================================
INSERT INTO users (username, password_hash, email, role) 
VALUES ('admin', crypt('admin123', gen_salt('bf')), 'admin@iws.local', 'admin')
ON CONFLICT (username) DO NOTHING;

INSERT INTO configuration (key, value, description) VALUES
('scanning_profiles', '{"moderate":{"threads":50},"aggressive":{"threads":100},"stealth":{"threads":10},"comprehensive":{"threads":30}}', 'Default scanning profiles'),
('max_threads', '50', 'Maximum concurrent scanning threads'),
('default_timeout', '30', 'Default request timeout in seconds'),
('default_profile', '"moderate"', 'Default scanning profile')
ON CONFLICT (key) DO NOTHING;
