# models/ml_scanner.py
# IWS v1.0 - ML Scanner
# Machine learning models untuk klasifikasi dan deteksi pola keamanan

import numpy as np
import pickle
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass
from sklearn.ensemble import RandomForestClassifier, GradientBoostingClassifier
from sklearn.svm import SVC
from sklearn.model_selection import GridSearchCV
from sklearn.preprocessing import StandardScaler
import joblib


@dataclass
class MlPrediction:
    label: str
    confidence: float
    probabilities: Dict[str, float]
    feature_importance: Optional[Dict[str, float]] = None


class MlScanner:
    def __init__(self):
        self.models: Dict[str, Any] = {}
        self.scaler = StandardScaler()
        self._trained = False
        self._feature_names: List[str] = []

    def train(self, features: np.ndarray, labels: np.ndarray, feature_names: Optional[List[str]] = None) -> Dict[str, float]:
        """Train multiple models dan pilih yang terbaik"""
        self._feature_names = feature_names or [f"f_{i}" for i in range(features.shape[1])]
        X = self.scaler.fit_transform(features)

        models = {
            "random_forest": RandomForestClassifier(n_estimators=100, max_depth=10, random_state=42),
            "svm": SVC(kernel="rbf", probability=True, random_state=42),
            "gradient_boost": GradientBoostingClassifier(n_estimators=100, random_state=42),
        }

        results = {}
        for name, model in models.items():
            model.fit(X, labels)
            score = model.score(X, labels)
            self.models[name] = model
            results[name] = score

        self._trained = True
        return results

    def predict(self, features: np.ndarray) -> MlPrediction:
        """Prediksi menggunakan ensemble voting"""
        if not self._trained:
            return MlPrediction(label="unknown", confidence=0.0, probabilities={})

        X = self.scaler.transform(features.reshape(1, -1)) if features.ndim == 1 else self.scaler.transform(features)
        votes = []
        probs_sum = {}

        for name, model in self.models.items():
            pred = model.predict(X)[0]
            votes.append(pred)
            if hasattr(model, "predict_proba"):
                for i, cls in enumerate(model.classes_):
                    probs_sum[cls] = probs_sum.get(cls, 0.0) + model.predict_proba(X)[0][i]

        # Majority voting
        from collections import Counter
        label = Counter(votes).most_common(1)[0][0]
        total = sum(probs_sum.values()) or 1.0
        probabilities = {k: v / total for k, v in probs_sum.items()}
        confidence = probabilities.get(label, max(probabilities.values()) if probabilities else 0.0)

        # Feature importance dari Random Forest jika tersedia
        importance = None
        if "random_forest" in self.models:
            imp = self.models["random_forest"].feature_importances_
            importance = dict(zip(self._feature_names, imp.tolist()))

        return MlPrediction(label=str(label), confidence=float(confidence), probabilities=probabilities, feature_importance=importance)

    def predict_batch(self, features: np.ndarray) -> List[MlPrediction]:
        return [self.predict(f) for f in features]

    def tune_hyperparameters(self, features: np.ndarray, labels: np.ndarray) -> Dict[str, Any]:
        """Hyperparameter tuning dengan GridSearchCV"""
        X = self.scaler.fit_transform(features)
        param_grid = {"n_estimators": [50, 100, 200], "max_depth": [5, 10, 15, None]}
        grid = GridSearchCV(RandomForestClassifier(random_state=42), param_grid, cv=3, scoring="f1_weighted")
        grid.fit(X, labels)
        self.models["random_forest"] = grid.best_estimator_
        self._trained = True
        return grid.best_params_

    def save(self, path: str) -> None:
        """Save model ke file"""
        data = {"models": self.models, "scaler": self.scaler, "feature_names": self._feature_names}
        joblib.dump(data, path)

    def load(self, path: str) -> None:
        """Load model dari file"""
        data = joblib.load(path)
        self.models = data["models"]
        self.scaler = data["scaler"]
        self._feature_names = data.get("feature_names", [])
        self._trained = True

    def explain(self, features: np.ndarray) -> Dict[str, float]:
        """Jelaskan prediksi dengan feature importance"""
        if "random_forest" not in self.models:
            return {}
        importance = self.models["random_forest"].feature_importances_
        return dict(zip(self._feature_names, importance.tolist()))


if __name__ == "__main__":
    # Quick test
    X = np.random.rand(100, 5)
    y = np.random.choice(["benign", "malicious", "suspicious"], 100)

    scanner = MlScanner()
    results = scanner.train(X, y, feature_names=["resp_time", "ttl", "content_len", "headers_count", "ssl_score"])
    print(f"Training results: {results}")

    pred = scanner.predict(np.random.rand(5))
    print(f"Prediction: {pred}")
