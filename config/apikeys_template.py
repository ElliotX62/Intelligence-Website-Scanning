# config/apikeys_template.py
# IWS v1.0 - API Keys Template
# Template dan manajemen API keys untuk third-party services

import os
import logging
from typing import Dict, List, Optional, Any
from dataclasses import dataclass, field
from enum import Enum

logger = logging.getLogger(__name__)


# ============================================================
# API KEY STATUS
# ============================================================

class ApiKeyStatus(str, Enum):
    VALID = "valid"
    INVALID = "invalid"
    MISSING = "missing"
    PLACEHOLDER = "placeholder"
    RATE_LIMITED = "rate_limited"
    EXPIRED = "expired"
    UNKNOWN = "unknown"

    def __str__(self) -> str:
        return self.value

    def is_usable(self) -> bool:
        return self in (ApiKeyStatus.VALID,)


# ============================================================
# API KEYS
# ============================================================

@dataclass
class ApiKeys:
    # Shodan — Network intelligence & IoT scanning
    SHODAN_API_KEY: str = "YOUR_API_KEY_HERE"

    # Censys — Internet-wide scanning data
    CENSYS_API_ID: str = "YOUR_API_KEY_HERE"
    CENSYS_API_SECRET: str = "YOUR_API_KEY_HERE"

    # VirusTotal — Malware & URL scanning
    VIRUSTOTAL_API_KEY: str = "YOUR_API_KEY_HERE"

    # AlienVault OTX — Open threat intelligence exchange
    ALIENVAULT_API_KEY: str = "YOUR_API_KEY_HERE"

    # URLScan — Website behavior analysis
    URLSCAN_API_KEY: str = "YOUR_API_KEY_HERE"

    # SecurityTrails — DNS history & domain intelligence
    SECURITYTRAILS_API_KEY: str = "YOUR_API_KEY_HERE"

    # GreyHat Warfare — Fast threat intelligence access
    GREYHAT_API_KEY: str = "YOUR_API_KEY_HERE"

    # Google Safe Browsing
    GOOGLE_SAFE_BROWSING_KEY: str = "YOUR_API_KEY_HERE"

    # Web of Trust (WOT)
    WOT_API_KEY: str = "YOUR_API_KEY_HERE"

    # IBM X-Force Exchange
    IBM_XFORCE_API_KEY: str = "YOUR_API_KEY_HERE"
    IBM_XFORCE_API_PASSWORD: str = "YOUR_API_KEY_HERE"

    # MISP — Malware Information Sharing Platform
    MISP_API_KEY: str = "YOUR_API_KEY_HERE"
    MISP_BASE_URL: str = "https://misp.example.com"

    # DNSDB — Historical DNS data
    DNSDB_API_KEY: str = "YOUR_API_KEY_HERE"

    # crt.sh — Certificate transparency (no API key needed, rate-limited)
    CRTSH_API_ENABLED: bool = True

    # Additional metadata
    _status_cache: Dict[str, ApiKeyStatus] = field(default_factory=dict, repr=False)
    _last_validated: Optional[str] = field(default=None, repr=False)

    @classmethod
    def load_from_env(cls, prefix: str = "IWS_") -> "ApiKeys":
        """Load API keys dari environment variables dengan prefix IWS_"""
        keys = cls()

        env_mapping = {
            "SHODAN_API_KEY": "SHODAN_API_KEY",
            "CENSYS_API_ID": "CENSYS_API_ID",
            "CENSYS_API_SECRET": "CENSYS_API_SECRET",
            "VIRUSTOTAL_API_KEY": "VIRUSTOTAL_API_KEY",
            "ALIENVAULT_API_KEY": "ALIENVAULT_API_KEY",
            "URLSCAN_API_KEY": "URLSCAN_API_KEY",
            "SECURITYTRAILS_API_KEY": "SECURITYTRAILS_API_KEY",
            "GREYHAT_API_KEY": "GREYHAT_API_KEY",
            "GOOGLE_SAFE_BROWSING_KEY": "GOOGLE_SAFE_BROWSING_KEY",
            "WOT_API_KEY": "WOT_API_KEY",
            "IBM_XFORCE_API_KEY": "IBM_XFORCE_API_KEY",
            "IBM_XFORCE_API_PASSWORD": "IBM_XFORCE_API_PASSWORD",
            "MISP_API_KEY": "MISP_API_KEY",
            "MISP_BASE_URL": "MISP_BASE_URL",
            "DNSDB_API_KEY": "DNSDB_API_KEY",
        }

        for attr_name, env_name in env_mapping.items():
            env_value = os.environ.get(f"{prefix}{env_name}")
            if env_value:
                setattr(keys, attr_name, env_value)

        return keys

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "ApiKeys":
        """Load dari dictionary"""
        valid_fields = {f for f in cls.__dataclass_fields__ if not f.startswith("_")}
        filtered = {k: v for k, v in data.items() if k in valid_fields}
        return cls(**filtered)

    def validate_keys(self) -> Dict[str, ApiKeyStatus]:
        """Validasi semua API keys — cek format dan placeholder"""
        import datetime

        statuses = {}

        # Shodan: 32 karakter alfanumerik
        statuses["shodan"] = self._validate_key(
            self.SHODAN_API_KEY, min_len=32, max_len=64, pattern="shodan"
        )

        # Censys: ID (36 chars UUID) + Secret
        statuses["censys"] = self._validate_key(
            self.CENSYS_API_ID, min_len=32, max_len=64, pattern="censys_id"
        )
        if statuses["censys"] == ApiKeyStatus.VALID:
            statuses["censys_secret"] = self._validate_key(
                self.CENSYS_API_SECRET, min_len=32, max_len=64, pattern="generic"
            )

        # VirusTotal: 64 karakter
        statuses["virustotal"] = self._validate_key(
            self.VIRUSTOTAL_API_KEY, min_len=64, max_len=64, pattern="virustotal"
        )

        # AlienVault OTX
        statuses["alienvault"] = self._validate_key(
            self.ALIENVAULT_API_KEY, min_len=32, max_len=128, pattern="alienvault"
        )

        # URLScan
        statuses["urlscan"] = self._validate_key(
            self.URLSCAN_API_KEY, min_len=32, max_len=64, pattern="urlscan"
        )

        # SecurityTrails
        statuses["securitytrails"] = self._validate_key(
            self.SECURITYTRAILS_API_KEY, min_len=32, max_len=64, pattern="generic"
        )

        # GreyHat Warfare
        statuses["greyhat"] = self._validate_key(
            self.GREYHAT_API_KEY, min_len=16, max_len=64, pattern="generic"
        )

        # Google Safe Browsing
        statuses["google_safe_browsing"] = self._validate_key(
            self.GOOGLE_SAFE_BROWSING_KEY, min_len=32, max_len=64, pattern="google"
        )

        # WOT
        statuses["wot"] = self._validate_key(
            self.WOT_API_KEY, min_len=32, max_len=64, pattern="generic"
        )

        # IBM X-Force
        statuses["ibm_xforce"] = self._validate_key(
            self.IBM_XFORCE_API_KEY, min_len=32, max_len=64, pattern="generic"
        )

        # MISP
        statuses["misp"] = self._validate_key(
            self.MISP_API_KEY, min_len=32, max_len=128, pattern="generic"
        )

        # DNSDB
        statuses["dnsdb"] = self._validate_key(
            self.DNSDB_API_KEY, min_len=32, max_len=64, pattern="generic"
        )

        self._status_cache = statuses
        self._last_validated = datetime.datetime.utcnow().isoformat()
        return statuses

    def _validate_key(
        self, key: str, min_len: int = 16, max_len: int = 128, pattern: str = "generic"
    ) -> ApiKeyStatus:
        """Validasi individual API key"""
        if not key:
            return ApiKeyStatus.MISSING

        if key == "YOUR_API_KEY_HERE":
            return ApiKeyStatus.PLACEHOLDER

        if len(key) < min_len:
            return ApiKeyStatus.INVALID

        if len(key) > max_len:
            return ApiKeyStatus.INVALID

        # Pattern-specific validation
        if pattern == "shodan":
            if not key.isalnum():
                return ApiKeyStatus.INVALID
        elif pattern == "virustotal":
            if not all(c in "0123456789abcdefABCDEF" for c in key):
                return ApiKeyStatus.INVALID
        elif pattern == "google":
            if not key.startswith("AIza"):
                return ApiKeyStatus.INVALID
        elif pattern == "alienvault":
            if len(key) < 32:
                return ApiKeyStatus.INVALID
        elif pattern == "censys_id":
            if "-" not in key and len(key) != 36:
                return ApiKeyStatus.INVALID

        return ApiKeyStatus.VALID

    def get_active_services(self) -> List[str]:
        """Dapatkan list services dengan API key valid"""
        statuses = self._status_cache or self.validate_keys()
        return [
            name for name, status in statuses.items()
            if status.is_usable()
        ]

    def get_missing_services(self) -> List[str]:
        """Dapatkan list services yang belum dikonfigurasi"""
        statuses = self._status_cache or self.validate_keys()
        return [
            name for name, status in statuses.items()
            if status in (ApiKeyStatus.MISSING, ApiKeyStatus.PLACEHOLDER)
        ]

    def get_service_status(self, service_name: str) -> ApiKeyStatus:
        """Dapatkan status spesifik service"""
        statuses = self._status_cache or self.validate_keys()
        return statuses.get(service_name.lower(), ApiKeyStatus.UNKNOWN)

    def mask_keys(self) -> Dict[str, str]:
        """Mask API keys untuk logging (hanya tampilkan 4 karakter pertama & terakhir)"""
        masked = {}
        for field_name in self.__dataclass_fields__:
            if field_name.startswith("_"):
                continue
            value = getattr(self, field_name, "")
            if isinstance(value, str) and len(value) > 8:
                masked[field_name] = f"{value[:4]}...{value[-4:]}"
            elif isinstance(value, str):
                masked[field_name] = "***"
            else:
                masked[field_name] = str(value)
        return masked

    def to_dict(self) -> Dict[str, Any]:
        """Konversi ke dictionary (tanpa internal fields)"""
        return {
            f: getattr(self, f)
            for f in self.__dataclass_fields__
            if not f.startswith("_")
        }

    def is_any_service_available(self) -> bool:
        """Cek apakah minimal ada 1 service yang valid"""
        return len(self.get_active_services()) > 0

    def are_all_services_available(self) -> bool:
        """Cek apakah semua service valid"""
        return len(self.get_missing_services()) == 0

    def __repr__(self) -> str:
        active = len(self.get_active_services())
        total = len([f for f in self.__dataclass_fields__ if not f.startswith("_")])
        return f"ApiKeys(active={active}/{total})"


# ============================================================
# API KEY VALIDATOR
# ============================================================

class ApiKeyValidator:
    """Validator untuk API keys dengan format checking"""

    @staticmethod
    def validate_shodan(key: str) -> bool:
        return len(key) >= 32 and key.isalnum()

    @staticmethod
    def validate_virustotal(key: str) -> bool:
        return len(key) == 64 and all(c in "0123456789abcdefABCDEF" for c in key)

    @staticmethod
    def validate_google_api(key: str) -> bool:
        return key.startswith("AIza") and len(key) >= 35

    @staticmethod
    def validate_stripe_key(key: str) -> bool:
        return (key.startswith("sk_live_") or key.startswith("sk_test_")) and len(key) >= 24

    @staticmethod
    def validate_github_token(key: str) -> bool:
        return key.startswith("ghp_") and len(key) >= 36

    @staticmethod
    def validate_aws_key(key: str) -> bool:
        return key.startswith("AKIA") and len(key) == 20

    @staticmethod
    def is_placeholder(key: str) -> bool:
        placeholders = [
            "YOUR_API_KEY_HERE", "your_api_key_here",
            "TODO", "todo", "changeme", "CHANGE_ME",
            "api_key", "API_KEY", "", "null", "None",
        ]
        return key in placeholders


# ============================================================
# API KEY ROTATION
# ============================================================

class ApiKeyRotation:
    """Manajemen rotasi API key untuk multiple keys per service"""

    def __init__(self):
        self._keys: Dict[str, List[str]] = {}
        self._current_index: Dict[str, int] = {}
        self._cooldowns: Dict[str, float] = {}

    def add_keys(self, service: str, keys: List[str]) -> None:
        """Tambah multiple API keys untuk satu service"""
        self._keys[service] = [k for k in keys if k and k != "YOUR_API_KEY_HERE"]
        self._current_index[service] = 0

    def get_next_key(self, service: str) -> Optional[str]:
        """Dapatkan API key berikutnya (round-robin rotation)"""
        keys = self._keys.get(service, [])
        if not keys:
            return None

        import time
        cooldown_key = f"{service}:cooldown"
        if cooldown_key in self._cooldowns:
            if time.time() < self._cooldowns[cooldown_key]:
                return None

        idx = self._current_index.get(service, 0)
        key = keys[idx % len(keys)]
        self._current_index[service] = (idx + 1) % len(keys)
        return key

    def mark_rate_limited(self, service: str, cooldown_seconds: float = 60.0) -> None:
        """Tandai service sedang rate-limited"""
        import time
        self._cooldowns[f"{service}:cooldown"] = time.time() + cooldown_seconds

    def get_available_services(self) -> List[str]:
        """Dapatkan services yang punya keys tersedia"""
        import time
        available = []
        for service in self._keys:
            cooldown_key = f"{service}:cooldown"
            if cooldown_key not in self._cooldowns or time.time() >= self._cooldowns[cooldown_key]:
                if self._keys[service]:
                    available.append(service)
        return available

    def count_keys(self, service: str) -> int:
        """Hitung jumlah keys untuk service"""
        return len(self._keys.get(service, []))


# ============================================================
# UNIT TESTS
# ============================================================

if __name__ == "__main__":
    import pytest
    import sys

    class TestApiKeys:
        def test_default_placeholder(self):
            keys = ApiKeys()
            assert keys.SHODAN_API_KEY == "YOUR_API_KEY_HERE"

        def test_load_from_env(self, monkeypatch):
            monkeypatch.setenv("IWS_SHODAN_API_KEY", "test_shodan_key_12345678901234567890")
            keys = ApiKeys.load_from_env()
            assert keys.SHODAN_API_KEY == "test_shodan_key_12345678901234567890"

        def test_validate_placeholder(self):
            keys = ApiKeys()
            statuses = keys.validate_keys()
            assert statuses["shodan"] == ApiKeyStatus.PLACEHOLDER

        def test_validate_valid_key(self):
            keys = ApiKeys()
            keys.SHODAN_API_KEY = "a" * 32
            statuses = keys.validate_keys()
            assert statuses["shodan"] == ApiKeyStatus.VALID

        def test_validate_virustotal_format(self):
            keys = ApiKeys()
            keys.VIRUSTOTAL_API_KEY = "g" * 64
            statuses = keys.validate_keys()
            assert statuses["virustotal"] == ApiKeyStatus.INVALID

            keys.VIRUSTOTAL_API_KEY = "a" * 64
            statuses = keys.validate_keys()
            assert statuses["virustotal"] == ApiKeyStatus.VALID

        def test_get_active_services(self):
            keys = ApiKeys()
            keys.SHODAN_API_KEY = "a" * 32
            active = keys.get_active_services()
            assert "shodan" in active

        def test_get_missing_services(self):
            keys = ApiKeys()
            missing = keys.get_missing_services()
            assert len(missing) > 0

        def test_mask_keys(self):
            keys = ApiKeys()
            keys.SHODAN_API_KEY = "abcdefghijklmnopqrstuvwxyz123456"
            masked = keys.mask_keys()
            assert "abcd...3456" in masked["SHODAN_API_KEY"]

        def test_is_any_service_available(self):
            keys = ApiKeys()
            assert not keys.is_any_service_available()
            keys.SHODAN_API_KEY = "a" * 32
            assert keys.is_any_service_available()

        def test_to_dict(self):
            keys = ApiKeys()
            d = keys.to_dict()
            assert "SHODAN_API_KEY" in d
            assert not d["SHODAN_API_KEY"].startswith("_")

    class TestApiKeyValidator:
        def test_is_placeholder(self):
            assert ApiKeyValidator.is_placeholder("YOUR_API_KEY_HERE")
            assert ApiKeyValidator.is_placeholder("TODO")
            assert not ApiKeyValidator.is_placeholder("real_key_12345")

        def test_validate_google_api(self):
            assert ApiKeyValidator.validate_google_api("AIza" + "a" * 31)
            assert not ApiKeyValidator.validate_google_api("not_google_key")

        def test_validate_github_token(self):
            assert ApiKeyValidator.validate_github_token("ghp_" + "a" * 32)
            assert not ApiKeyValidator.validate_github_token("not_github_token")

    class TestApiKeyRotation:
        def test_add_and_get_keys(self):
            rotation = ApiKeyRotation()
            rotation.add_keys("shodan", ["key1", "key2", "key3"])
            assert rotation.count_keys("shodan") == 3
            assert rotation.get_next_key("shodan") == "key1"
            assert rotation.get_next_key("shodan") == "key2"
            assert rotation.get_next_key("shodan") == "key3"
            assert rotation.get_next_key("shodan") == "key1"

        def test_rate_limited(self):
            rotation = ApiKeyRotation()
            rotation.add_keys("virustotal", ["vt_key_1", "vt_key_2"])
            rotation.mark_rate_limited("virustotal", cooldown_seconds=999.0)
            assert rotation.get_next_key("virustotal") is None

        def test_get_available_services(self):
            rotation = ApiKeyRotation()
            rotation.add_keys("shodan", ["key1"])
            rotation.add_keys("virustotal", [])
            available = rotation.get_available_services()
            assert "shodan" in available
            assert "virustotal" not in available

    sys.exit(pytest.main([__file__, "-v", "--tb=short"]))
