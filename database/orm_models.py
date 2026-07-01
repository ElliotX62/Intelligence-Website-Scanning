# database/orm_models.py
# IWS v1.0 - ORM Models
# Mendefinisikan SQLAlchemy models untuk database

from datetime import datetime
from typing import Optional, List, Dict, Any
from sqlalchemy import (
    Column, String, Integer, Float, Boolean, DateTime, Text,
    ForeignKey, JSON, Enum as SAEnum, Index, UniqueConstraint
)
from sqlalchemy.dialects.postgresql import UUID, JSONB
from sqlalchemy.orm import relationship, declarative_base
import uuid
import enum

Base = declarative_base()

# ============================================================
# ENUMS
# ============================================================

class UserRole(str, enum.Enum):
    ADMIN = "admin"
    USER = "user"
    GUEST = "guest"

class ScanStatus(str, enum.Enum):
    PENDING = "pending"
    ACTIVE = "active"
    COMPLETED = "completed"
    FAILED = "failed"
    CANCELLED = "cancelled"

class Severity(str, enum.Enum):
    CRITICAL = "critical"
    HIGH = "high"
    MEDIUM = "medium"
    LOW = "low"
    INFO = "info"

class VulnStatus(str, enum.Enum):
    OPEN = "open"
    IN_PROGRESS = "in_progress"
    FIXED = "fixed"
    WONT_FIX = "wont_fix"
    FALSE_POSITIVE = "false_positive"
    DUPLICATE = "duplicate"

# ============================================================
# MODELS
# ============================================================

class User(Base):
    __tablename__ = "users"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    username = Column(String(255), unique=True, nullable=False, index=True)
    password_hash = Column(String(255), nullable=False)
    email = Column(String(255), unique=True, nullable=False, index=True)
    role = Column(String(20), default=UserRole.USER.value)
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow)
    last_login = Column(DateTime(timezone=True))
    is_active = Column(Boolean, default=True)
    api_key = Column(String(64), unique=True)
    settings = Column(JSONB, default=dict)

    scans = relationship("ScanResult", back_populates="user", cascade="all, delete-orphan")

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": str(self.id), "username": self.username, "email": self.email,
            "role": self.role, "is_active": self.is_active,
            "created_at": self.created_at.isoformat() if self.created_at else None,
        }


class ScanResult(Base):
    __tablename__ = "scan_results"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    user_id = Column(UUID(as_uuid=True), ForeignKey("users.id", ondelete="CASCADE"), index=True)
    target_url = Column(String(2048), nullable=False)
    target_ip = Column(String(45))
    scan_profile = Column(String(50), default="moderate")
    started_at = Column(DateTime(timezone=True), default=datetime.utcnow)
    completed_at = Column(DateTime(timezone=True))
    status = Column(String(20), default=ScanStatus.PENDING.value, index=True)
    result = Column(JSONB, default=dict)
    summary = Column(Text)
    risk_score = Column(Float)
    scan_metadata = Column(JSONB, default=dict)

    user = relationship("User", back_populates="scans")
    vulnerabilities = relationship("Vulnerability", back_populates="scan", cascade="all, delete-orphan")
    findings = relationship("Finding", back_populates="scan", cascade="all, delete-orphan")

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": str(self.id), "target_url": self.target_url,
            "status": self.status, "risk_score": self.risk_score,
            "started_at": self.started_at.isoformat() if self.started_at else None,
        }


class Vulnerability(Base):
    __tablename__ = "vulnerabilities"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    scan_id = Column(UUID(as_uuid=True), ForeignKey("scan_results.id", ondelete="CASCADE"), index=True)
    cve_id = Column(String(50), index=True)
    title = Column(String(500), nullable=False)
    description = Column(Text)
    severity = Column(String(10), index=True)
    cvss_score = Column(Float)
    cvss_vector = Column(String(100))
    affected_component = Column(String(500))
    affected_version = Column(String(100))
    fixed_version = Column(String(100))
    remediation = Column(Text)
    status = Column(String(20), default=VulnStatus.OPEN.value)
    discovered_at = Column(DateTime(timezone=True), default=datetime.utcnow)
    fixed_at = Column(DateTime(timezone=True))

    scan = relationship("ScanResult", back_populates="vulnerabilities")

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": str(self.id), "cve_id": self.cve_id, "title": self.title,
            "severity": self.severity, "cvss_score": self.cvss_score, "status": self.status,
        }


class Finding(Base):
    __tablename__ = "findings"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    scan_id = Column(UUID(as_uuid=True), ForeignKey("scan_results.id", ondelete="CASCADE"), index=True)
    module_name = Column(String(100))
    finding_type = Column(String(100))
    title = Column(String(500), nullable=False)
    description = Column(Text)
    severity = Column(String(10))
    confidence = Column(Float, default=0.5)
    location = Column(Text)
    evidence = Column(JSONB)
    remediation = Column(Text)
    discovered_at = Column(DateTime(timezone=True), default=datetime.utcnow)

    scan = relationship("ScanResult", back_populates="findings")

    def to_dict(self) -> Dict[str, Any]:
        return {
            "id": str(self.id), "title": self.title, "finding_type": self.finding_type,
            "severity": self.severity, "confidence": self.confidence,
        }


class AgentState(Base):
    __tablename__ = "agent_states"

    id = Column(Integer, primary_key=True, autoincrement=True)
    agent_id = Column(UUID(as_uuid=True), nullable=False, index=True)
    agent_name = Column(String(100), nullable=False)
    agent_type = Column(String(50))
    state = Column(String(20), default="uninitialized")
    last_heartbeat = Column(DateTime(timezone=True))
    state_data = Column(JSONB, default=dict)
    updated_at = Column(DateTime(timezone=True), default=datetime.utcnow)


class Configuration(Base):
    __tablename__ = "configuration"

    id = Column(Integer, primary_key=True, autoincrement=True)
    key = Column(String(255), unique=True, nullable=False, index=True)
    value = Column(JSONB, nullable=False)
    description = Column(Text)
    updated_at = Column(DateTime(timezone=True), default=datetime.utcnow)


class AuditLog(Base):
    __tablename__ = "audit_log"

    id = Column(Integer, primary_key=True, autoincrement=True)
    user_id = Column(UUID(as_uuid=True), ForeignKey("users.id"))
    action = Column(String(100), nullable=False)
    resource_type = Column(String(50))
    resource_id = Column(String(100))
    details = Column(JSONB, default=dict)
    ip_address = Column(String(45))
    user_agent = Column(Text)
    created_at = Column(DateTime(timezone=True), default=datetime.utcnow)


# ============================================================
# SESSION FACTORY
# ============================================================

def create_tables(engine):
    """Buat semua tabel"""
    Base.metadata.create_all(engine)

def drop_tables(engine):
    """Drop semua tabel"""
    Base.metadata.drop_all(engine)
