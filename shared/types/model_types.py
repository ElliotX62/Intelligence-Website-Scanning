# shared/types/model_types.py
# IWS v1.0 - Model Types
# Mendefinisikan tipe data untuk AI/ML model management

from __future__ import annotations
from dataclasses import dataclass, field, asdict
from typing import Dict, List, Optional, Any, Tuple, Union
from datetime import datetime
from enum import Enum
import numpy as np
import hashlib
import json


# ============================================================
# ENUMS
# ============================================================

class ModelType(str, Enum):
    RANDOM_FOREST = "random_forest"
    SVM = "svm"
    GRADIENT_BOOSTING = "gradient_boosting"
    ISOLATION_FOREST = "isolation_forest"
    KMEANS = "kmeans"
    PCA = "pca"
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

    def is_llm(self) -> bool:
        return self in (
            ModelType.LLM_LLAMA3,
            ModelType.LLM_MISTRAL,
            ModelType.LLM_PHI2,
        )

    def is_ml(self) -> bool:
        return self in (
            ModelType.RANDOM_FOREST,
            ModelType.SVM,
            ModelType.GRADIENT_BOOSTING,
            ModelType.ISOLATION_FOREST,
            ModelType.KMEANS,
            ModelType.PCA,
        )

    def is_nlp(self) -> bool:
        return self in (
            ModelType.BERT_NER,
            ModelType.BART_SUMMARIZATION,
            ModelType.SENTENCE_TRANSFORMER,
        )


class ModelStatus(str, Enum):
    UNLOADED = "unloaded"
    LOADING = "loading"
    LOADED = "loaded"
    ERROR = "error"
    UNLOADING = "unloading"
    TRAINING = "training"
    EVALUATING = "evaluating"

    def __str__(self) -> str:
        return self.value

    def is_ready(self) -> bool:
        return self == ModelStatus.LOADED

    def is_busy(self) -> bool:
        return self in (ModelStatus.LOADING, ModelStatus.TRAINING, ModelStatus.EVALUATING)


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


class Device(str, Enum):
    CPU = "cpu"
    CUDA = "cuda"
    MPS = "mps"
    AUTO = "auto"

    def __str__(self) -> str:
        return self.value


# ============================================================
# DATA CLASSES
# ============================================================

@dataclass
class FeatureVector:
    values: List[float]
    feature_names: Optional[List[str]] = None
    metadata: Dict[str, Any] = field(default_factory=dict)
    _hash: Optional[int] = field(default=None, repr=False)

    def validate(self) -> Tuple[bool, Optional[str]]:
        if not self.values:
            return False, "Feature vector is empty"
        if any(np.isnan(v) for v in self.values):
            return False, "Feature vector contains NaN values"
        if any(np.isinf(v) for v in self.values):
            return False, "Feature vector contains infinite values"
        if self.feature_names and len(self.feature_names) != len(self.values):
            return False, (
                f"Feature names ({len(self.feature_names)}) != values ({len(self.values)})"
            )
        return True, None

    def to_numpy(self) -> np.ndarray:
        return np.array(self.values, dtype=np.float32)

    def to_dict(self) -> Dict[str, float]:
        if self.feature_names:
            return dict(zip(self.feature_names, self.values))
        return {f"feature_{i}": v for i, v in enumerate(self.values)}

    def normalize(self, method: str = "minmax") -> "FeatureVector":
        arr = np.array(self.values)
        if method == "minmax":
            min_val = arr.min()
            max_val = arr.max()
            if max_val - min_val > 0:
                normalized = (arr - min_val) / (max_val - min_val)
            else:
                normalized = arr
        elif method == "zscore":
            mean = arr.mean()
            std = arr.std()
            if std > 0:
                normalized = (arr - mean) / std
            else:
                normalized = arr
        else:
            normalized = arr
        return FeatureVector(
            values=normalized.tolist(),
            feature_names=self.feature_names,
            metadata={**self.metadata, "normalized": method},
        )

    def __len__(self) -> int:
        return len(self.values)

    def __getitem__(self, index: int) -> float:
        return self.values[index]

    def __hash__(self) -> int:
        if self._hash is None:
            self._hash = hash(tuple(self.values))
        return self._hash

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, FeatureVector):
            return NotImplemented
        return self.values == other.values


@dataclass
class PredictionResult:
    label: str
    confidence: float
    probabilities: Dict[str, float]
    raw_output: Optional[Any] = None
    explanation: Optional["Explanation"] = None
    inference_time_ms: float = 0.0
    model_name: str = ""
    model_version: str = ""
    timestamp: str = field(default_factory=lambda: datetime.utcnow().isoformat())
    metadata: Dict[str, Any] = field(default_factory=dict)

    def validate(self) -> Tuple[bool, Optional[str]]:
        if not self.label:
            return False, "Label is empty"
        if self.confidence < 0.0 or self.confidence > 1.0:
            return False, f"Confidence out of range: {self.confidence}"
        if self.probabilities:
            total = sum(self.probabilities.values())
            if abs(total - 1.0) > 0.02:
                return False, f"Probabilities sum to {total}, expected ~1.0"
            if self.label not in self.probabilities:
                return False, f"Label '{self.label}' not found in probabilities"
        return True, None

    def get_top_k(self, k: int = 3) -> List[Tuple[str, float]]:
        sorted_probs = sorted(
            self.probabilities.items(), key=lambda x: x[1], reverse=True
        )
        return sorted_probs[:k]

    def is_high_confidence(self, threshold: float = 0.8) -> bool:
        return self.confidence >= threshold

    def entropy(self) -> float:
        probs = np.array(list(self.probabilities.values()))
        probs = probs[probs > 0]
        return -float(np.sum(probs * np.log2(probs)))

    def to_dict(self) -> Dict[str, Any]:
        return {
            "label": self.label,
            "confidence": self.confidence,
            "probabilities": self.probabilities,
            "inference_time_ms": self.inference_time_ms,
            "model_name": self.model_name,
            "model_version": self.model_version,
            "timestamp": self.timestamp,
        }

    def __repr__(self) -> str:
        return (
            f"PredictionResult(label='{self.label}', "
            f"confidence={self.confidence:.3f}, "
            f"time={self.inference_time_ms:.1f}ms)"
        )


@dataclass
class Explanation:
    method: str
    feature_importance: Dict[str, float]
    top_features: List[Tuple[str, float]]
    summary: str
    details: Dict[str, Any] = field(default_factory=dict)
    shap_values: Optional[List[float]] = None
    lime_explanation: Optional[Dict[str, Any]] = None
    counterfactual: Optional[Dict[str, Any]] = None

    def to_dict(self) -> Dict[str, Any]:
        return {
            "method": self.method,
            "feature_importance": self.feature_importance,
            "top_features": self.top_features,
            "summary": self.summary,
            "details": self.details,
        }

    def most_important_features(self, n: int = 5) -> List[str]:
        sorted_features = sorted(
            self.feature_importance.items(), key=lambda x: abs(x[1]), reverse=True
        )
        return [f[0] for f in sorted_features[:n]]


@dataclass
class TrainingData:
    features: List[FeatureVector]
    labels: List[str]
    weights: Optional[List[float]] = None
    metadata: Dict[str, Any] = field(default_factory=dict)
    _validated: bool = field(default=False, repr=False)

    def validate(self) -> Tuple[bool, Optional[str]]:
        if not self.features:
            return False, "Features list is empty"
        if not self.labels:
            return False, "Labels list is empty"
        if len(self.features) != len(self.labels):
            return False, (
                f"Features ({len(self.features)}) != labels ({len(self.labels)})"
            )
        if self.weights and len(self.weights) != len(self.features):
            return False, (
                f"Weights ({len(self.weights)}) != features ({len(self.features)})"
            )
        feature_len = len(self.features[0])
        for i, fv in enumerate(self.features):
            if len(fv) != feature_len:
                return False, f"Feature {i} length {len(fv)} != expected {feature_len}"
            valid, err = fv.validate()
            if not valid:
                return False, f"Feature {i}: {err}"
        self._validated = True
        return True, None

    def get_class_distribution(self) -> Dict[str, int]:
        dist: Dict[str, int] = {}
        for label in self.labels:
            dist[label] = dist.get(label, 0) + 1
        return dist

    def is_balanced(self, threshold: float = 0.3) -> bool:
        dist = self.get_class_distribution()
        if len(dist) < 2:
            return True
        counts = list(dist.values())
        max_count = max(counts)
        min_count = min(counts)
        if max_count == 0:
            return True
        return (min_count / max_count) >= threshold

    def split(
        self, train_ratio: float = 0.8, shuffle: bool = True
    ) -> Tuple["TrainingData", "TrainingData"]:
        indices = list(range(len(self.features)))
        if shuffle:
            import random
            random.shuffle(indices)
        split_idx = int(len(indices) * train_ratio)
        train_indices = indices[:split_idx]
        test_indices = indices[split_idx:]
        return (
            TrainingData(
                features=[self.features[i] for i in train_indices],
                labels=[self.labels[i] for i in train_indices],
                weights=(
                    [self.weights[i] for i in train_indices]
                    if self.weights else None
                ),
                metadata={**self.metadata, "split": "train"},
            ),
            TrainingData(
                features=[self.features[i] for i in test_indices],
                labels=[self.labels[i] for i in test_indices],
                weights=(
                    [self.weights[i] for i in test_indices]
                    if self.weights else None
                ),
                metadata={**self.metadata, "split": "test"},
            ),
        )

    def feature_matrix(self) -> np.ndarray:
        return np.array([fv.values for fv in self.features], dtype=np.float32)

    def label_array(self) -> np.ndarray:
        return np.array(self.labels)

    def __len__(self) -> int:
        return len(self.features)

    def __repr__(self) -> str:
        return f"TrainingData(samples={len(self)}, features={len(self.features[0]) if self.features else 0}, labels={len(set(self.labels))})"


@dataclass
class ModelConfig:
    model_type: ModelType
    hyperparameters: Dict[str, Any]
    path: Optional[str] = None
    version: str = "1.0.0"
    device: Device = Device.CPU
    batch_size: int = 32
    max_sequence_length: int = 512
    cache_enabled: bool = True
    cache_ttl_seconds: int = 3600
    timeout_seconds: int = 30
    num_workers: int = 1
    seed: int = 42
    custom_config: Dict[str, Any] = field(default_factory=dict)

    def validate(self) -> Tuple[bool, Optional[str]]:
        if self.batch_size < 1 or self.batch_size > 1024:
            return False, f"Invalid batch_size: {self.batch_size} (must be 1-1024)"
        if self.max_sequence_length < 1 or self.max_sequence_length > 8192:
            return False, (
                f"Invalid max_sequence_length: {self.max_sequence_length} (must be 1-8192)"
            )
        if self.timeout_seconds < 1 or self.timeout_seconds > 3600:
            return False, (
                f"Invalid timeout_seconds: {self.timeout_seconds} (must be 1-3600)"
            )
        if self.num_workers < 0 or self.num_workers > 16:
            return False, f"Invalid num_workers: {self.num_workers} (must be 0-16)"
        return True, None

    def to_dict(self) -> Dict[str, Any]:
        return {
            "model_type": self.model_type.value,
            "version": self.version,
            "device": self.device.value,
            "batch_size": self.batch_size,
            "max_sequence_length": self.max_sequence_length,
            "seed": self.seed,
        }

    def cache_key(self) -> str:
        raw = (
            f"{self.model_type.value}:{self.version}:"
            f"{json.dumps(self.hyperparameters, sort_keys=True)}"
        )
        return hashlib.md5(raw.encode()).hexdigest()

    def __repr__(self) -> str:
        return (
            f"ModelConfig(type={self.model_type.value}, "
            f"device={self.device.value}, version={self.version})"
        )


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
    early_stopped: bool = False
    stopped_at_epoch: int = 0
    timestamp: str = field(default_factory=lambda: datetime.utcnow().isoformat())

    def to_dict(self) -> Dict[str, Any]:
        return {
            "model_config": self.model_config.to_dict(),
            "metrics": self.metrics.to_dict(),
            "training_time_seconds": self.training_time_seconds,
            "epochs_completed": self.epochs_completed,
            "convergence_reached": self.convergence_reached,
            "best_epoch": self.best_epoch,
            "best_loss": self.best_loss,
            "early_stopped": self.early_stopped,
            "warnings": self.warnings,
        }

    def is_successful(self) -> bool:
        return self.convergence_reached and self.metrics.is_performing_well()

    def __repr__(self) -> str:
        return (
            f"TrainingResult(epochs={self.epochs_completed}, "
            f"converged={self.convergence_reached}, "
            f"time={self.training_time_seconds:.0f}s)"
        )


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
    support: Optional[int] = None
    custom_metrics: Dict[str, float] = field(default_factory=dict)

    def validate(self) -> Tuple[bool, Optional[str]]:
        for metric_name in ("accuracy", "precision", "recall", "f1"):
            value = getattr(self, metric_name, None)
            if value is not None and (value < 0.0 or value > 1.0):
                return False, f"{metric_name} out of range [0,1]: {value}"
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
        if self.mae is not None:
            result["mae"] = self.mae
        if self.rmse is not None:
            result["rmse"] = self.rmse
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

    def __repr__(self) -> str:
        return self.get_summary()


# ============================================================
# INFERENCE CACHE
# ============================================================

@dataclass
class CacheEntry:
    prediction: PredictionResult
    cached_at: datetime = field(default_factory=datetime.utcnow)
    access_count: int = 1
    last_accessed: datetime = field(default_factory=datetime.utcnow)

    def is_expired(self, ttl_seconds: int) -> bool:
        elapsed = (datetime.utcnow() - self.cached_at).total_seconds()
        return elapsed > ttl_seconds

    def access(self) -> None:
        self.access_count += 1
        self.last_accessed = datetime.utcnow()


class InferenceCache:
    def __init__(self, max_size: int = 1000, ttl_seconds: int = 3600):
        self._cache: Dict[str, CacheEntry] = {}
        self._max_size = max_size
        self._ttl_seconds = ttl_seconds
        self._hits: int = 0
        self._misses: int = 0
        self._evictions: int = 0

    def get(self, key: str) -> Optional[PredictionResult]:
        entry = self._cache.get(key)
        if entry is None:
            self._misses += 1
            return None
        if entry.is_expired(self._ttl_seconds):
            del self._cache[key]
            self._evictions += 1
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
            key=lambda k: (
                self._cache[k].last_accessed,
                -self._cache[k].access_count,
            ),
        )
        del self._cache[lru_key]
        self._evictions += 1

    def clear(self) -> None:
        self._cache.clear()
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def remove(self, key: str) -> bool:
        if key in self._cache:
            del self._cache[key]
            return True
        return False

    def get_hit_rate(self) -> float:
        total = self._hits + self._misses
        if total == 0:
            return 0.0
        return self._hits / total

    def size(self) -> int:
        return len(self._cache)

    def is_full(self) -> bool:
        return len(self._cache) >= self._max_size

    def get_stats(self) -> Dict[str, Any]:
        return {
            "size": self.size(),
            "max_size": self._max_size,
            "hits": self._hits,
            "misses": self._misses,
            "evictions": self._evictions,
            "hit_rate": self.get_hit_rate(),
            "ttl_seconds": self._ttl_seconds,
        }

    def keys(self) -> List[str]:
        return list(self._cache.keys())

    def __len__(self) -> int:
        return self.size()

    def __contains__(self, key: str) -> bool:
        entry = self._cache.get(key)
        if entry is None:
            return False
        if entry.is_expired(self._ttl_seconds):
            del self._cache[key]
            return False
        return True


# ============================================================
# UNIT TESTS
# ============================================================

if __name__ == "__main__":
    import pytest
    import sys

    class TestFeatureVector:
        def test_validate_valid(self):
            fv = FeatureVector([1.0, 2.0, 3.0])
            valid, err = fv.validate()
            assert valid
            assert err is None

        def test_validate_empty(self):
            fv = FeatureVector([])
            valid, err = fv.validate()
            assert not valid

        def test_validate_nan(self):
            fv = FeatureVector([1.0, float("nan")])
            valid, err = fv.validate()
            assert not valid
            assert "NaN" in err

        def test_to_numpy(self):
            fv = FeatureVector([1.0, 2.0, 3.0])
            arr = fv.to_numpy()
            assert arr.shape == (3,)
            assert arr.dtype == np.float32

        def test_normalize_minmax(self):
            fv = FeatureVector([0.0, 5.0, 10.0])
            normalized = fv.normalize("minmax")
            assert normalized.values[0] == 0.0
            assert abs(normalized.values[2] - 1.0) < 0.001

        def test_hash_equality(self):
            fv1 = FeatureVector([1.0, 2.0, 3.0])
            fv2 = FeatureVector([1.0, 2.0, 3.0])
            fv3 = FeatureVector([3.0, 2.0, 1.0])
            assert hash(fv1) == hash(fv2)
            assert hash(fv1) != hash(fv3)
            assert fv1 == fv2

    class TestPredictionResult:
        def test_validate_valid(self):
            pr = PredictionResult(
                label="benign",
                confidence=0.95,
                probabilities={"benign": 0.95, "malicious": 0.05},
            )
            valid, _ = pr.validate()
            assert valid

        def test_validate_bad_confidence(self):
            pr = PredictionResult(label="x", confidence=1.5, probabilities={})
            valid, err = pr.validate()
            assert not valid
            assert "range" in err.lower()

        def test_entropy(self):
            pr = PredictionResult(
                label="a",
                confidence=0.5,
                probabilities={"a": 0.5, "b": 0.5},
            )
            assert abs(pr.entropy() - 1.0) < 0.01

            pr2 = PredictionResult(
                label="a",
                confidence=0.99,
                probabilities={"a": 0.99, "b": 0.01},
            )
            assert pr2.entropy() < 0.1

        def test_get_top_k(self):
            pr = PredictionResult(
                label="a",
                confidence=0.4,
                probabilities={"a": 0.4, "b": 0.35, "c": 0.25},
            )
            top2 = pr.get_top_k(2)
            assert len(top2) == 2
            assert top2[0][0] == "a"

    class TestTrainingData:
        def test_split(self):
            features = [FeatureVector([float(i)]) for i in range(100)]
            labels = [f"class_{i % 3}" for i in range(100)]
            data = TrainingData(features, labels)
            train, test = data.split(0.8)
            assert len(train) == 80
            assert len(test) == 20
            assert train.metadata.get("split") == "train"

        def test_validate_mismatch(self):
            data = TrainingData(
                [FeatureVector([1.0])],
                ["a", "b"],
            )
            valid, _ = data.validate()
            assert not valid

        def test_class_distribution(self):
            fv = FeatureVector([1.0])
            data = TrainingData([fv, fv, fv, fv], ["a", "a", "b", "c"])
            dist = data.get_class_distribution()
            assert dist == {"a": 2, "b": 1, "c": 1}

        def test_is_balanced(self):
            fv = FeatureVector([1.0])
            balanced = TrainingData([fv, fv, fv, fv], ["a", "a", "b", "b"])
            assert balanced.is_balanced(0.3)

            imbalanced = TrainingData([fv, fv, fv], ["a", "a", "a"])
            assert not imbalanced.is_balanced(0.3)

    class TestModelConfig:
        def test_validate_valid(self):
            config = ModelConfig(
                model_type=ModelType.RANDOM_FOREST,
                hyperparameters={"n_estimators": 100},
            )
            valid, _ = config.validate()
            assert valid

        def test_validate_bad_batch(self):
            config = ModelConfig(
                model_type=ModelType.SVM,
                hyperparameters={},
                batch_size=0,
            )
            valid, err = config.validate()
            assert not valid

        def test_cache_key_consistency(self):
            config1 = ModelConfig(
                model_type=ModelType.GRADIENT_BOOSTING,
                hyperparameters={"lr": 0.01},
            )
            config2 = ModelConfig(
                model_type=ModelType.GRADIENT_BOOSTING,
                hyperparameters={"lr": 0.01},
            )
            assert config1.cache_key() == config2.cache_key()

    class TestEvaluationMetrics:
        def test_is_performing_well(self):
            good = EvaluationMetrics(0.9, 0.9, 0.9, 0.9)
            assert good.is_performing_well()

            bad = EvaluationMetrics(0.5, 0.5, 0.5, 0.5)
            assert not bad.is_performing_well()

        def test_summary(self):
            metrics = EvaluationMetrics(0.85, 0.88, 0.82, 0.85)
            summary = metrics.get_summary()
            assert "0.850" in summary

    class TestInferenceCache:
        def test_put_get(self):
            cache = InferenceCache(max_size=10, ttl_seconds=60)
            pr = PredictionResult(label="test", confidence=0.5, probabilities={})
            cache.put("k1", pr)
            result = cache.get("k1")
            assert result is not None
            assert result.label == "test"

        def test_miss(self):
            cache = InferenceCache()
            assert cache.get("nonexistent") is None

        def test_eviction_lru(self):
            cache = InferenceCache(max_size=2, ttl_seconds=600)
            for i in range(3):
                pr = PredictionResult(label=str(i), confidence=0.5, probabilities={})
                cache.put(str(i), pr)
            assert cache.size() == 2

        def test_hit_rate(self):
            cache = InferenceCache(max_size=10, ttl_seconds=600)
            pr = PredictionResult(label="x", confidence=0.5, probabilities={})
            cache.put("k", pr)
            cache.get("k")
            cache.get("k")
            cache.get("missing")
            assert cache.get_hit_rate() == 2.0 / 3.0

        def test_contains(self):
            cache = InferenceCache(max_size=10, ttl_seconds=600)
            pr = PredictionResult(label="x", confidence=0.5, probabilities={})
            cache.put("k", pr)
            assert "k" in cache
            assert "zzz" not in cache

        def test_expiration(self):
            cache = InferenceCache(max_size=10, ttl_seconds=0)
            pr = PredictionResult(label="x", confidence=0.5, probabilities={})
            cache.put("k", pr)
            import time
            time.sleep(0.1)
            assert cache.get("k") is None

    sys.exit(pytest.main([__file__, "-v", "--tb=short"]))
