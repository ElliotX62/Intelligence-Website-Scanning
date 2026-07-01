# shared/contracts/model_contract.py
# IWS v1.0 - Model Contract
# Mendefinisikan kontrak formal untuk semua AI/ML models

from __future__ import annotations
from abc import ABC, abstractmethod
from dataclasses import dataclass, field, asdict
from typing import Dict, List, Optional, Any, Tuple, Union, Callable
from datetime import datetime
from enum import Enum
import json
import hashlib
import numpy as np
import logging
import functools

logger = logging.getLogger(__name__)

# ============================================================
# MODEL TYPES & ENUMS
# ============================================================

class ModelType(str, Enum):
    RANDOM_FOREST = "random_forest"
    SVM = "svm"
    GRADIENT_BOOSTING = "gradient_boosting"
    ISOLATION_FOREST = "isolation_forest"
    BERT_NER = "bert_ner"
    BART_SUMMARIZATION = "bart_summarization"
    LLM_LLAMA3 = "llama3"
    LLM_MISTRAL = "mistral"
    LLM_PHI2 = "phi2"
    SENTENCE_TRANSFORMER = "sentence_transformer"
    CODEBERT = "codebert"
    CODET5 = "codet5"
    CUSTOM = "custom"

    def __str__(self) -> str:
        return self.value

class ModelStatus(str, Enum):
    UNLOADED = "unloaded"
    LOADING = "loading"
    LOADED = "loaded"
    ERROR = "error"
    UNLOADING = "unloading"

    def __str__(self) -> str:
        return self.value

class TaskType(str, Enum):
    CLASSIFICATION = "classification"
    REGRESSION = "regression"
    NER = "ner"
    SUMMARIZATION = "summarization"
    SENTIMENT = "sentiment"
    EMBEDDING = "embedding"
    GENERATION = "generation"
    ANOMALY_DETECTION = "anomaly_detection"
    PATTERN_MATCHING = "pattern_matching"
    CUSTOM = "custom"

    def __str__(self) -> str:
        return self.value

class Severity(str, Enum):
    INFO = "info"
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"

    def __str__(self) -> str:
        return self.value

    def to_score(self) -> float:
        mapping = {
            Severity.INFO: 0.0,
            Severity.LOW: 2.5,
            Severity.MEDIUM: 5.0,
            Severity.HIGH: 7.5,
            Severity.CRITICAL: 10.0,
        }
        return mapping[self]

    @classmethod
    def from_score(cls, score: float) -> "Severity":
        if score >= 9.0:
            return cls.CRITICAL
        elif score >= 7.0:
            return cls.HIGH
        elif score >= 4.0:
            return cls.MEDIUM
        elif score >= 0.1:
            return cls.LOW
        return cls.INFO

# ============================================================
# VALIDATE INPUT DECORATOR (SPEC FIX)
# ============================================================

def validate_input(func: Callable) -> Callable:
    """Decorator untuk memvalidasi input FeatureVector sebelum inference.
    
    Dapat di-apply ke method predict(), explain(), atau method lain
    yang menerima FeatureVector sebagai argument pertama.
    
    Raises:
        ModelError: Jika validasi input gagal
    
    Usage:
        @validate_input
        def predict(self, features: FeatureVector) -> PredictionResult:
            ...
    """
    @functools.wraps(func)
    def wrapper(self, features, *args, **kwargs):
        # Handle single FeatureVector
        if isinstance(features, FeatureVector):
            valid, err = features.validate()
            if not valid:
                raise ModelError.invalid_input(
                    f"Input validation failed for {func.__name__}: {err}",
                    details={"feature_count": 1, "error": err},
                )
        # Handle list of FeatureVectors
        elif isinstance(features, list):
            if not features:
                raise ModelError.invalid_input(
                    f"Empty feature list passed to {func.__name__}",
                    details={"feature_count": 0},
                )
            feature_len = len(features[0])
            for i, fv in enumerate(features):
                if not isinstance(fv, FeatureVector):
                    raise ModelError.invalid_input(
                        f"Element {i} is not a FeatureVector in {func.__name__}",
                        details={"index": i, "type": type(fv).__name__},
                    )
                if len(fv) != feature_len:
                    raise ModelError.invalid_input(
                        f"Feature {i} has inconsistent length in {func.__name__}: "
                        f"{len(fv)} vs {feature_len}",
                        details={"index": i, "length": len(fv), "expected": feature_len},
                    )
                valid, err = fv.validate()
                if not valid:
                    raise ModelError.invalid_input(
                        f"Feature {i} validation failed for {func.__name__}: {err}",
                        details={"index": i, "error": err},
                    )
        else:
            raise ModelError.invalid_input(
                f"Unsupported feature type in {func.__name__}: {type(features).__name__}. "
                f"Expected FeatureVector or list of FeatureVector.",
                details={"type": type(features).__name__},
            )
        
        return func(self, features, *args, **kwargs)
    return wrapper


def validate_training_data(func: Callable) -> Callable:
    """Decorator untuk memvalidasi TrainingData sebelum training/evaluasi.
    
    Raises:
        ModelError: Jika data training tidak valid
    """
    @functools.wraps(func)
    def wrapper(self, data, *args, **kwargs):
        if not isinstance(data, TrainingData):
            raise ModelError.invalid_input(
                f"Expected TrainingData, got {type(data).__name__} in {func.__name__}",
                details={"type": type(data).__name__},
            )
        valid, err = data.validate()
        if not valid:
            raise ModelError.invalid_input(
                f"TrainingData validation failed for {func.__name__}: {err}",
                details={"samples": len(data), "error": err},
            )
        return func(self, data, *args, **kwargs)
    return wrapper


def require_model_loaded(func: Callable) -> Callable:
    """Decorator untuk memastikan model sudah di-load sebelum method dipanggil.
    
    Raises:
        ModelError: Jika model belum di-load
    """
    @functools.wraps(func)
    def wrapper(self, *args, **kwargs):
        if self._status != ModelStatus.LOADED:
            raise ModelError.model_not_loaded(
                f"Cannot call {func.__name__}: model is {self._status.value}"
            )
        return func(self, *args, **kwargs)
    return wrapper

# ============================================================
# DATA CLASSES
# ============================================================

@dataclass
class FeatureVector:
    values: List[float]
    feature_names: Optional[List[str]] = None
    metadata: Dict[str, Any] = field(default_factory=dict)

    def validate(self) -> Tuple[bool, Optional[str]]:
        if not self.values:
            return False, "Feature vector is empty"
        if any(np.isnan(v) for v in self.values):
            return False, "Feature vector contains NaN values"
        if any(np.isinf(v) for v in self.values):
            return False, "Feature vector contains infinite values"
        if self.feature_names and len(self.feature_names) != len(self.values):
            return False, f"Feature names ({len(self.feature_names)}) != values ({len(self.values)})"
        return True, None

    def to_numpy(self) -> np.ndarray:
        return np.array(self.values, dtype=np.float32)

    def to_dict(self) -> Dict[str, float]:
        if self.feature_names:
            return dict(zip(self.feature_names, self.values))
        return {f"feature_{i}": v for i, v in enumerate(self.values)}

    def __len__(self) -> int:
        return len(self.values)

    def __getitem__(self, index: int) -> float:
        return self.values[index]

    def __hash__(self) -> int:
        return hash(tuple(self.values))

@dataclass
class PredictionResult:
    label: str
    confidence: float
    probabilities: Dict[str, float]
    raw_output: Optional[Any] = None
    explanation: Optional["Explanation"] = None
    inference_time_ms: float = 0.0
    model_version: str = ""
    timestamp: str = field(default_factory=lambda: datetime.utcnow().isoformat())

    def validate(self) -> Tuple[bool, Optional[str]]:
        if not self.label:
            return False, "Label is empty"
        if self.confidence < 0.0 or self.confidence > 1.0:
            return False, f"Confidence out of range: {self.confidence}"
        if self.probabilities:
            total = sum(self.probabilities.values())
            if abs(total - 1.0) > 0.01:
                return False, f"Probabilities sum to {total}, expected 1.0"
            if self.label not in self.probabilities:
                return False, f"Label '{self.label}' not in probabilities"
        return True, None

    def get_top_k(self, k: int = 3) -> List[Tuple[str, float]]:
        sorted_probs = sorted(
            self.probabilities.items(), key=lambda x: x[1], reverse=True
        )
        return sorted_probs[:k]

    def is_high_confidence(self, threshold: float = 0.8) -> bool:
        return self.confidence >= threshold

    def to_dict(self) -> Dict[str, Any]:
        return {
            "label": self.label,
            "confidence": self.confidence,
            "probabilities": self.probabilities,
            "inference_time_ms": self.inference_time_ms,
            "model_version": self.model_version,
            "timestamp": self.timestamp,
        }

@dataclass
class Explanation:
    method: str
    feature_importance: Dict[str, float]
    top_features: List[Tuple[str, float]]
    summary: str
    details: Dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "method": self.method,
            "feature_importance": self.feature_importance,
            "top_features": self.top_features,
            "summary": self.summary,
            "details": self.details,
        }

@dataclass
class TrainingData:
    features: List[FeatureVector]
    labels: List[str]
    weights: Optional[List[float]] = None
    metadata: Dict[str, Any] = field(default_factory=dict)

    def validate(self) -> Tuple[bool, Optional[str]]:
        if not self.features:
            return False, "Features list is empty"
        if not self.labels:
            return False, "Labels list is empty"
        if len(self.features) != len(self.labels):
            return False, f"Features ({len(self.features)}) != labels ({len(self.labels)})"
        if self.weights and len(self.weights) != len(self.features):
            return False, f"Weights ({len(self.weights)}) != features ({len(self.features)})"

        feature_len = len(self.features[0])
        for i, fv in enumerate(self.features):
            if len(fv) != feature_len:
                return False, f"Feature {i} has length {len(fv)}, expected {feature_len}"
            valid, err = fv.validate()
            if not valid:
                return False, f"Feature {i}: {err}"

        return True, None

    def get_class_distribution(self) -> Dict[str, int]:
        dist = {}
        for label in self.labels:
            dist[label] = dist.get(label, 0) + 1
        return dist

    def is_balanced(self, threshold: float = 0.3) -> bool:
        dist = self.get_class_distribution()
        if not dist:
            return True
        max_count = max(dist.values())
        min_count = min(dist.values())
        if max_count == 0:
            return True
        return (min_count / max_count) >= threshold

    def split(self, train_ratio: float = 0.8) -> Tuple["TrainingData", "TrainingData"]:
        split_idx = int(len(self.features) * train_ratio)
        train_features = self.features[:split_idx]
        train_labels = self.labels[:split_idx]
        train_weights = self.weights[:split_idx] if self.weights else None

        test_features = self.features[split_idx:]
        test_labels = self.labels[split_idx:]
        test_weights = self.weights[split_idx:] if self.weights else None

        return (
            TrainingData(train_features, train_labels, train_weights, self.metadata.copy()),
            TrainingData(test_features, test_labels, test_weights, self.metadata.copy()),
        )

    def __len__(self) -> int:
        return len(self.features)

@dataclass
class ModelConfig:
    model_type: ModelType
    hyperparameters: Dict[str, Any]
    path: Optional[str] = None
    version: str = "1.0.0"
    device: str = "cpu"
    batch_size: int = 32
    max_sequence_length: int = 512
    cache_enabled: bool = True
    cache_ttl_seconds: int = 3600
    timeout_seconds: int = 30
    custom_config: Dict[str, Any] = field(default_factory=dict)

    def validate(self) -> Tuple[bool, Optional[str]]:
        if self.batch_size < 1 or self.batch_size > 1024:
            return False, f"Invalid batch_size: {self.batch_size}"
        if self.max_sequence_length < 1 or self.max_sequence_length > 8192:
            return False, f"Invalid max_sequence_length: {self.max_sequence_length}"
        if self.timeout_seconds < 1 or self.timeout_seconds > 3600:
            return False, f"Invalid timeout_seconds: {self.timeout_seconds}"
        if self.device not in ["cpu", "cuda", "mps"]:
            return False, f"Unsupported device: {self.device}"
        return True, None

    def to_dict(self) -> Dict[str, Any]:
        return {
            "model_type": self.model_type.value,
            "hyperparameters": self.hyperparameters,
            "version": self.version,
            "device": self.device,
            "batch_size": self.batch_size,
        }

    def cache_key(self) -> str:
        raw = f"{self.model_type}:{self.version}:{json.dumps(self.hyperparameters, sort_keys=True)}"
        return hashlib.md5(raw.encode()).hexdigest()

@dataclass
class TrainingResult:
    model_config: ModelConfig
    metrics: "EvaluationMetrics"
    training_time_seconds: float
    epochs_completed: int
    convergence_reached: bool
    best_epoch: int
    best_loss: float
    history: Dict[str, List[float]] = field(default_factory=dict)
    warnings: List[str] = field(default_factory=list)
    artifacts_path: Optional[str] = None

    def to_dict(self) -> Dict[str, Any]:
        return {
            "model_config": self.model_config.to_dict(),
            "metrics": self.metrics.to_dict(),
            "training_time_seconds": self.training_time_seconds,
            "epochs_completed": self.epochs_completed,
            "convergence_reached": self.convergence_reached,
            "best_epoch": self.best_epoch,
            "best_loss": self.best_loss,
            "warnings": self.warnings,
        }

@dataclass
class EvaluationMetrics:
    accuracy: float
    precision: float
    recall: float
    f1: float
    auc_roc: Optional[float] = None
    confusion_matrix: Optional[List[List[int]]] = None
    log_loss: Optional[float] = None
    mae: Optional[float] = None
    mse: Optional[float] = None
    rmse: Optional[float] = None
    r2: Optional[float] = None
    custom_metrics: Dict[str, float] = field(default_factory=dict)

    def validate(self) -> Tuple[bool, Optional[str]]:
        for metric_name in ["accuracy", "precision", "recall", "f1"]:
            value = getattr(self, metric_name, None)
            if value is not None and (value < 0.0 or value > 1.0):
                return False, f"{metric_name} out of range: {value}"
        return True, None

    def to_dict(self) -> Dict[str, Any]:
        result = {
            "accuracy": self.accuracy,
            "precision": self.precision,
            "recall": self.recall,
            "f1": self.f1,
        }
        if self.auc_roc is not None:
            result["auc_roc"] = self.auc_roc
        if self.confusion_matrix is not None:
            result["confusion_matrix"] = self.confusion_matrix
        if self.log_loss is not None:
            result["log_loss"] = self.log_loss
        result.update(self.custom_metrics)
        return result

    def is_performing_well(self, threshold: float = 0.8) -> bool:
        return self.f1 >= threshold and self.accuracy >= threshold

    def get_summary(self) -> str:
        return (
            f"Accuracy={self.accuracy:.3f}, "
            f"Precision={self.precision:.3f}, "
            f"Recall={self.recall:.3f}, "
            f"F1={self.f1:.3f}"
        )

# ============================================================
# INFERENCE CACHE
# ============================================================

@dataclass
class CacheEntry:
    prediction: PredictionResult
    cached_at: datetime = field(default_factory=datetime.utcnow)
    access_count: int = 1

    def is_expired(self, ttl_seconds: int) -> bool:
        elapsed = (datetime.utcnow() - self.cached_at).total_seconds()
        return elapsed > ttl_seconds

    def access(self) -> None:
        self.access_count += 1

class InferenceCache:
    def __init__(self, max_size: int = 1000, ttl_seconds: int = 3600):
        self._cache: Dict[str, CacheEntry] = {}
        self._max_size = max_size
        self._ttl_seconds = ttl_seconds
        self._hits = 0
        self._misses = 0

    def get(self, key: str) -> Optional[PredictionResult]:
        entry = self._cache.get(key)
        if entry is None:
            self._misses += 1
            return None
        if entry.is_expired(self._ttl_seconds):
            del self._cache[key]
            self._misses += 1
            return None
        entry.access()
        self._hits += 1
        return entry.prediction

    def put(self, key: str, prediction: PredictionResult) -> None:
        if len(self._cache) >= self._max_size:
            self._evict_lru()
        self._cache[key] = CacheEntry(prediction=prediction)

    def _evict_lru(self) -> None:
        if not self._cache:
            return
        lru_key = min(
            self._cache.keys(),
            key=lambda k: (self._cache[k].cached_at, -self._cache[k].access_count),
        )
        del self._cache[lru_key]

    def clear(self) -> None:
        self._cache.clear()
        self._hits = 0
        self._misses = 0

    def get_hit_rate(self) -> float:
        total = self._hits + self._misses
        if total == 0:
            return 0.0
        return self._hits / total

    def size(self) -> int:
        return len(self._cache)

    def get_stats(self) -> Dict[str, Any]:
        return {
            "size": self.size(),
            "max_size": self._max_size,
            "hits": self._hits,
            "misses": self._misses,
            "hit_rate": self.get_hit_rate(),
            "ttl_seconds": self._ttl_seconds,
        }

# ============================================================
# ABSTRACT MODEL CONTRACT
# ============================================================

class ModelContract(ABC):
    """Base contract untuk semua AI/ML models dalam IWS.
    
    Semua model HARUS mengimplementasikan semua method abstract di bawah.
    Tidak boleh ada method yang return NotImplemented atau raise NotImplementedError.
    
    Decorators yang tersedia:
        @validate_input - validasi FeatureVector sebelum predict/explain
        @validate_training_data - validasi TrainingData sebelum train/evaluate
        @require_model_loaded - pastikan model sudah di-load
    """

    def __init__(self, config: ModelConfig):
        self._config = config
        self._status = ModelStatus.UNLOADED
        self._cache = InferenceCache(
            max_size=config.cache_enabled and 1000 or 0,
            ttl_seconds=config.cache_ttl_seconds,
        )
        self._model: Any = None
        self._loaded_at: Optional[datetime] = None
        self._inference_count = 0
        self._total_inference_time = 0.0

    @abstractmethod
    def load_model(self) -> None:
        """Load model dari path atau download dari registry.
        
        Raises:
            ModelError: Jika model gagal di-load
        """
        ...

    @abstractmethod
    def predict(self, features: Union[FeatureVector, List[FeatureVector]]) -> Union[PredictionResult, List[PredictionResult]]:
        """Melakukan inference/prediction.
        
        Subclasses HARUS menggunakan @validate_input dan @require_model_loaded decorator.
        
        Args:
            features: Single FeatureVector atau list of FeatureVectors
            
        Returns:
            PredictionResult atau list of PredictionResult
            
        Raises:
            ModelError: Jika inference gagal atau input invalid
        """
        ...

    @abstractmethod
    def explain(self, prediction: PredictionResult) -> Explanation:
        """Menjelaskan hasil prediksi.
        
        Subclasses HARUS menggunakan @require_model_loaded decorator.
        
        Args:
            prediction: Hasil prediksi yang ingin dijelaskan
            
        Returns:
            Explanation dengan feature importance dan summary
            
        Raises:
            ModelError: Jika explanation gagal
        """
        ...

    @abstractmethod
    def train(self, data: TrainingData) -> TrainingResult:
        """Melatih model dengan data training.
        
        Subclasses HARUS menggunakan @validate_training_data decorator.
        
        Args:
            data: TrainingData dengan features dan labels
            
        Returns:
            TrainingResult dengan metrics
            
        Raises:
            ModelError: Jika training gagal
        """
        ...

    @abstractmethod
    def evaluate(self, data: TrainingData) -> EvaluationMetrics:
        """Mengevaluasi performa model.
        
        Subclasses HARUS menggunakan @validate_training_data dan @require_model_loaded decorator.
        
        Args:
            data: TrainingData untuk evaluasi
            
        Returns:
            EvaluationMetrics
            
        Raises:
            ModelError: Jika evaluasi gagal
        """
        ...

    @abstractmethod
    def save_model(self, path: str) -> None:
        """Menyimpan model ke path.
        
        Args:
            path: Path untuk menyimpan model
            
        Raises:
            ModelError: Jika save gagal
        """
        ...

    def validate_input(self, features: FeatureVector) -> Tuple[bool, Optional[str]]:
        """Validasi input sebelum inference.
        
        Gunakan @validate_input decorator untuk validasi otomatis
        pada method predict() dan explain().
        """
        return features.validate()

    def get_status(self) -> ModelStatus:
        """Mendapatkan status model saat ini."""
        return self._status

    def get_config(self) -> ModelConfig:
        """Mendapatkan konfigurasi model."""
        return self._config

    def get_uptime_seconds(self) -> float:
        """Mendapatkan waktu sejak model di-load."""
        if self._loaded_at is None:
            return 0.0
        return (datetime.utcnow() - self._loaded_at).total_seconds()

    def get_inference_stats(self) -> Dict[str, Any]:
        """Mendapatkan statistik inference."""
        avg_time = (
            self._total_inference_time / self._inference_count
            if self._inference_count > 0
            else 0.0
        )
        return {
            "inference_count": self._inference_count,
            "total_inference_time": self._total_inference_time,
            "average_inference_time_ms": avg_time * 1000,
            "cache_stats": self._cache.get_stats(),
        }

    def _validate_features(self, features: Union[FeatureVector, List[FeatureVector]]) -> Tuple[bool, Optional[str]]:
        if isinstance(features, FeatureVector):
            return features.validate()
        if isinstance(features, list):
            for i, fv in enumerate(features):
                valid, err = fv.validate()
                if not valid:
                    return False, f"Feature {i}: {err}"
            if features and self._config.max_sequence_length:
                if len(features[0]) > self._config.max_sequence_length:
                    return False, f"Feature length ({len(features[0])}) exceeds max ({self._config.max_sequence_length})"
        return True, None

    def _start_timer(self) -> float:
        import time
        return time.time()

    def _stop_timer(self, start_time: float) -> float:
        import time
        elapsed = time.time() - start_time
        self._inference_count += 1
        self._total_inference_time += elapsed
        return elapsed

    def unload_model(self) -> None:
        """Unload model dari memory."""
        self._model = None
        self._status = ModelStatus.UNLOADED
        self._loaded_at = None
        self._cache.clear()

    def __repr__(self) -> str:
        return f"{self.__class__.__name__}(type={self._config.model_type}, status={self._status})"

# ============================================================
# MODEL ERROR
# ============================================================

class ModelError(Exception):
    """Base exception untuk semua model errors."""

    def __init__(
        self,
        message: str,
        code: str = "M6000",
        severity: str = "high",
        recoverable: bool = False,
        details: Optional[Dict[str, Any]] = None,
    ):
        super().__init__(message)
        self.message = message
        self.code = code
        self.severity = severity
        self.recoverable = recoverable
        self.details = details or {}
        self.timestamp = datetime.utcnow().isoformat()

    @classmethod
    def load_failed(cls, message: str, details: Optional[Dict[str, Any]] = None) -> "ModelError":
        return cls(message, code="M6001", severity="critical", recoverable=True, details=details)

    @classmethod
    def inference_failed(cls, message: str, details: Optional[Dict[str, Any]] = None) -> "ModelError":
        return cls(message, code="M6002", severity="high", recoverable=True, details=details)

    @classmethod
    def invalid_input(cls, message: str, details: Optional[Dict[str, Any]] = None) -> "ModelError":
        return cls(message, code="M6003", severity="medium", recoverable=False, details=details)

    @classmethod
    def training_failed(cls, message: str, details: Optional[Dict[str, Any]] = None) -> "ModelError":
        return cls(message, code="M6004", severity="high", recoverable=True, details=details)

    @classmethod
    def timeout(cls, message: str, details: Optional[Dict[str, Any]] = None) -> "ModelError":
        return cls(message, code="M6005", severity="medium", recoverable=True, details=details)

    @classmethod
    def model_not_loaded(cls, message: str = "Model not loaded") -> "ModelError":
        return cls(message, code="M6006", severity="high", recoverable=False)

    @classmethod
    def unsupported_task(cls, message: str) -> "ModelError":
        return cls(message, code="M6007", severity="medium", recoverable=False)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "code": self.code,
            "message": self.message,
            "severity": self.severity,
            "recoverable": self.recoverable,
            "details": self.details,
            "timestamp": self.timestamp,
        }

    def __str__(self) -> str:
        return f"[{self.code}] {self.message}"

# ============================================================
# MODEL REGISTRY
# ============================================================

class ModelRegistry:
    """Registry untuk tracking semua model yang di-load."""

    def __init__(self):
        self._models: Dict[str, ModelContract] = {}
        self._metadata: Dict[str, Dict[str, Any]] = {}

    def register(self, name: str, model: ModelContract, metadata: Optional[Dict[str, Any]] = None) -> None:
        if name in self._models:
            raise ModelError(
                f"Model '{name}' already registered",
                code="M6010",
                severity="low",
            )
        self._models[name] = model
        self._metadata[name] = metadata or {}

    def unregister(self, name: str) -> None:
        if name not in self._models:
            raise ModelError(
                f"Model '{name}' not found",
                code="M6011",
                severity="low",
            )
        model = self._models.pop(name)
        model.unload_model()
        self._metadata.pop(name, None)

    def get(self, name: str) -> ModelContract:
        if name not in self._models:
            raise ModelError(
                f"Model '{name}' not found in registry",
                code="M6011",
                severity="medium",
            )
        return self._models[name]

    def list_models(self) -> List[str]:
        return list(self._models.keys())

    def list_by_status(self, status: ModelStatus) -> List[str]:
        return [
            name
            for name, model in self._models.items()
            if model.get_status() == status
        ]

    def list_by_type(self, model_type: ModelType) -> List[str]:
        return [
            name
            for name, model in self._models.items()
            if model.get_config().model_type == model_type
        ]

    def count(self) -> int:
        return len(self._models)

    def get_all_stats(self) -> Dict[str, Dict[str, Any]]:
        return {
            name: {
                "status": model.get_status().value,
                "type": model.get_config().model_type.value,
                "inference_stats": model.get_inference_stats(),
                "uptime_seconds": model.get_uptime_seconds(),
                "metadata": self._metadata.get(name, {}),
            }
            for name, model in self._models.items()
        }

    def shutdown_all(self) -> None:
        for name in list(self._models.keys()):
            try:
                self.unregister(name)
            except Exception as e:
                logger.error(f"Error unloading model '{name}': {e}")

# ============================================================
# CONCRETE MODEL EXAMPLE (UNTUK TESTING DECORATOR)
# ============================================================

class DummyModel(ModelContract):
    """Model dummy untuk testing decorator — bukan untuk production."""
    
    def load_model(self) -> None:
        self._status = ModelStatus.LOADED
        self._loaded_at = datetime.utcnow()

    @validate_input
    @require_model_loaded
    def predict(self, features: Union[FeatureVector, List[FeatureVector]]) -> Union[PredictionResult, List[PredictionResult]]:
        self._start_timer()
        if isinstance(features, FeatureVector):
            result = PredictionResult(
                label="dummy",
                confidence=0.99,
                probabilities={"dummy": 0.99, "other": 0.01},
            )
        else:
            result = [
                PredictionResult(label="dummy", confidence=0.99, probabilities={"dummy": 0.99})
                for _ in features
            ]
        self._stop_timer(self._start_timer())
        return result

    @require_model_loaded
    def explain(self, prediction: PredictionResult) -> Explanation:
        return Explanation(
            method="dummy",
            feature_importance={},
            top_features=[],
            summary="Dummy explanation",
        )

    @validate_training_data
    def train(self, data: TrainingData) -> TrainingResult:
        return TrainingResult(
            model_config=self._config,
            metrics=EvaluationMetrics(1.0, 1.0, 1.0, 1.0),
            training_time_seconds=0.1,
            epochs_completed=1,
            convergence_reached=True,
            best_epoch=1,
            best_loss=0.0,
        )

    @validate_training_data
    @require_model_loaded
    def evaluate(self, data: TrainingData) -> EvaluationMetrics:
        return EvaluationMetrics(1.0, 1.0, 1.0, 1.0)

    def save_model(self, path: str) -> None:
        import os
        os.makedirs(os.path.dirname(path) if os.path.dirname(path) else ".", exist_ok=True)
        with open(path, "w") as f:
            json.dump({"type": "dummy", "version": self._config.version}, f)

# ============================================================
# UNIT TESTS
# ============================================================

if __name__ == "__main__":
    import pytest
    import sys

    class TestFeatureVector:
        def test_valid(self):
            fv = FeatureVector([1.0, 2.0, 3.0])
            valid, err = fv.validate()
            assert valid
            assert err is None

        def test_empty(self):
            fv = FeatureVector([])
            valid, err = fv.validate()
            assert not valid

        def test_nan(self):
            fv = FeatureVector([1.0, float("nan"), 3.0])
            valid, err = fv.validate()
            assert not valid

        def test_to_numpy(self):
            fv = FeatureVector([1.0, 2.0])
            arr = fv.to_numpy()
            assert arr.shape == (2,)
            assert arr.dtype == np.float32

        def test_with_names(self):
            fv = FeatureVector([1.0, 2.0], feature_names=["a", "b"])
            d = fv.to_dict()
            assert d == {"a": 1.0, "b": 2.0}

        def test_hash(self):
            fv1 = FeatureVector([1.0, 2.0])
            fv2 = FeatureVector([1.0, 2.0])
            fv3 = FeatureVector([2.0, 1.0])
            assert hash(fv1) == hash(fv2)
            assert hash(fv1) != hash(fv3)

    class TestPredictionResult:
        def test_valid(self):
            pr = PredictionResult(
                label="benign",
                confidence=0.9,
                probabilities={"benign": 0.9, "malicious": 0.1},
            )
            valid, err = pr.validate()
            assert valid

        def test_invalid_confidence(self):
            pr = PredictionResult(label="test", confidence=1.5, probabilities={})
            valid, err = pr.validate()
            assert not valid

        def test_top_k(self):
            pr = PredictionResult(
                label="a",
                confidence=0.5,
                probabilities={"a": 0.5, "b": 0.3, "c": 0.2},
            )
            top = pr.get_top_k(2)
            assert len(top) == 2
            assert top[0][0] == "a"

        def test_high_confidence(self):
            pr = PredictionResult(label="test", confidence=0.9, probabilities={})
            assert pr.is_high_confidence(0.8)
            assert not pr.is_high_confidence(0.95)

    class TestTrainingData:
        def test_validate(self):
            fv1 = FeatureVector([1.0, 2.0])
            fv2 = FeatureVector([3.0, 4.0])
            data = TrainingData([fv1, fv2], ["a", "b"])
            valid, err = data.validate()
            assert valid

        def test_mismatch_length(self):
            fv = FeatureVector([1.0])
            data = TrainingData([fv], ["a", "b"])
            valid, err = data.validate()
            assert not valid

        def test_split(self):
            features = [FeatureVector([float(i)]) for i in range(100)]
            labels = [str(i) for i in range(100)]
            data = TrainingData(features, labels)
            train, test = data.split(0.8)
            assert len(train) == 80
            assert len(test) == 20

        def test_class_distribution(self):
            fv = FeatureVector([1.0])
            data = TrainingData([fv, fv, fv], ["a", "a", "b"])
            dist = data.get_class_distribution()
            assert dist == {"a": 2, "b": 1}

    class TestInferenceCache:
        def test_put_get(self):
            cache = InferenceCache(max_size=10, ttl_seconds=60)
            pr = PredictionResult(label="test", confidence=0.5, probabilities={})
            cache.put("key1", pr)
            result = cache.get("key1")
            assert result is not None
            assert result.label == "test"

        def test_miss(self):
            cache = InferenceCache()
            result = cache.get("nonexistent")
            assert result is None

        def test_eviction(self):
            cache = InferenceCache(max_size=2, ttl_seconds=60)
            for i in range(3):
                pr = PredictionResult(label=str(i), confidence=0.5, probabilities={})
                cache.put(str(i), pr)
            assert cache.size() == 2

        def test_hit_rate(self):
            cache = InferenceCache(max_size=10, ttl_seconds=60)
            pr = PredictionResult(label="test", confidence=0.5, probabilities={})
            cache.put("key", pr)
            cache.get("key")
            cache.get("key")
            cache.get("missing")
            assert cache.get_hit_rate() == 2.0 / 3.0

    class TestModelError:
        def test_factory_methods(self):
            err = ModelError.load_failed("Cannot load")
            assert err.code == "M6001"
            assert err.severity == "critical"
            assert err.recoverable

            err = ModelError.invalid_input("Bad input")
            assert err.code == "M6003"
            assert not err.recoverable

        def test_to_dict(self):
            err = ModelError("test", code="M6999")
            d = err.to_dict()
            assert d["code"] == "M6999"
            assert "timestamp" in d

    class TestModelRegistry:
        def test_register_unregister(self):
            registry = ModelRegistry()
            config = ModelConfig(model_type=ModelType.CUSTOM, hyperparameters={})
            model = DummyModel(config)
            model.load_model()
            
            registry.register("dummy", model)
            assert registry.count() == 1
            
            registry.unregister("dummy")
            assert registry.count() == 0

        def test_duplicate_register_raises(self):
            registry = ModelRegistry()
            config = ModelConfig(model_type=ModelType.CUSTOM, hyperparameters={})
            model = DummyModel(config)
            model.load_model()
            
            registry.register("dummy", model)
            with pytest.raises(ModelError) as exc_info:
                registry.register("dummy", model)
            assert exc_info.value.code == "M6010"

        def test_get_nonexistent_raises(self):
            registry = ModelRegistry()
            with pytest.raises(ModelError) as exc_info:
                registry.get("nonexistent")
            assert exc_info.value.code == "M6011"

    class TestValidateInputDecorator:
        def test_valid_single_feature(self):
            config = ModelConfig(model_type=ModelType.CUSTOM, hyperparameters={})
            model = DummyModel(config)
            model.load_model()
            
            fv = FeatureVector([1.0, 2.0, 3.0])
            result = model.predict(fv)
            assert isinstance(result, PredictionResult)

        def test_valid_feature_list(self):
            config = ModelConfig(model_type=ModelType.CUSTOM, hyperparameters={})
            model = DummyModel(config)
            model.load_model()
            
            features = [FeatureVector([1.0]), FeatureVector([2.0])]
            result = model.predict(features)
            assert isinstance(result, list)
            assert len(result) == 2

        def test_invalid_nan_raises(self):
            config = ModelConfig(model_type=ModelType.CUSTOM, hyperparameters={})
            model = DummyModel(config)
            model.load_model()
            
            fv = FeatureVector([1.0, float("nan")])
            with pytest.raises(ModelError) as exc_info:
                model.predict(fv)
            assert exc_info.value.code == "M6003"

        def test_empty_feature_list_raises(self):
            config = ModelConfig(model_type=ModelType.CUSTOM, hyperparameters={})
            model = DummyModel(config)
            model.load_model()
            
            with pytest.raises(ModelError) as exc_info:
                model.predict([])
            assert exc_info.value.code == "M6003"

        def test_wrong_type_raises(self):
            config = ModelConfig(model_type=ModelType.CUSTOM, hyperparameters={})
            model = DummyModel(config)
            model.load_model()
            
            with pytest.raises(ModelError) as exc_info:
                model.predict("not a feature vector")
            assert exc_info.value.code == "M6003"

    class TestRequireModelLoadedDecorator:
        def test_not_loaded_raises(self):
            config = ModelConfig(model_type=ModelType.CUSTOM, hyperparameters={})
            model = DummyModel(config)
            # Tidak load model
            
            fv = FeatureVector([1.0])
            with pytest.raises(ModelError) as exc_info:
                model.predict(fv)
            assert exc_info.value.code == "M6006"

    class TestValidateTrainingDataDecorator:
        def test_valid_data(self):
            config = ModelConfig(model_type=ModelType.CUSTOM, hyperparameters={})
            model = DummyModel(config)
            
            fv = FeatureVector([1.0])
            data = TrainingData([fv], ["label"])
            result = model.train(data)
            assert result.convergence_reached

        def test_invalid_data_raises(self):
            config = ModelConfig(model_type=ModelType.CUSTOM, hyperparameters={})
            model = DummyModel(config)
            
            data = TrainingData([], [])
            with pytest.raises(ModelError) as exc_info:
                model.train(data)
            assert exc_info.value.code == "M6003"

        def test_wrong_type_raises(self):
            config = ModelConfig(model_type=ModelType.CUSTOM, hyperparameters={})
            model = DummyModel(config)
            
            with pytest.raises(ModelError) as exc_info:
                model.train("not training data")
            assert exc_info.value.code == "M6003"

    # Run tests
    sys.exit(pytest.main([__file__, "-v", "--tb=short"]))
