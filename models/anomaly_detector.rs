// models/anomaly_detector.rs
// IWS v1.0 - Anomaly Detector
// Mendeteksi anomali menggunakan Isolation Forest, Z-score, dan IQR

use std::collections::{HashMap, VecDeque};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyResult {
    pub is_anomaly: bool,
    pub score: f64,
    pub threshold: f64,
    pub method: String,
    pub details: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct IsolationTree {
    split_feature: usize,
    split_value: f64,
    left: Option<Box<IsolationTree>>,
    right: Option<Box<IsolationTree>>,
    size: usize,
}

impl IsolationTree {
    fn new(data: &[Vec<f64>], depth: usize, max_depth: usize) -> Self {
        let n = data.len();
        if depth >= max_depth || n <= 1 {
            return IsolationTree { split_feature: 0, split_value: 0.0, left: None, right: None, size: n };
        }

        let n_features = data[0].len();
        let split_feature = rand::random::<usize>() % n_features;
        let mut values: Vec<f64> = data.iter().map(|x| x[split_feature]).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let min_val = values[0];
        let max_val = values[n - 1];

        if (max_val - min_val).abs() < 1e-10 {
            return IsolationTree { split_feature, split_value: min_val, left: None, right: None, size: n };
        }

        let split_value = min_val + rand::random::<f64>() * (max_val - min_val);
        let (left_data, right_data): (Vec<Vec<f64>>, Vec<Vec<f64>>) = data.iter()
            .cloned()
            .partition(|x| x[split_feature] < split_value);

        IsolationTree {
            split_feature, split_value,
            left: if left_data.is_empty() { None } else { Some(Box::new(IsolationTree::new(&left_data, depth + 1, max_depth))) },
            right: if right_data.is_empty() { None } else { Some(Box::new(IsolationTree::new(&right_data, depth + 1, max_depth))) },
            size: n,
        }
    }

    fn path_length(&self, point: &[f64], current_depth: usize) -> f64 {
        if self.left.is_none() && self.right.is_none() {
            return current_depth as f64 + Self::c_factor(self.size);
        }
        if point[self.split_feature] < self.split_value {
            match &self.left {
                Some(left) => left.path_length(point, current_depth + 1),
                None => current_depth as f64 + Self::c_factor(self.size),
            }
        } else {
            match &self.right {
                Some(right) => right.path_length(point, current_depth + 1),
                None => current_depth as f64 + Self::c_factor(self.size),
            }
        }
    }

    fn c_factor(n: usize) -> f64 {
        if n <= 1 { return 0.0; }
        2.0 * ((n - 1) as f64).ln() + 0.5772156649 - (2.0 * (n - 1) as f64 / n as f64)
    }
}

pub struct AnomalyDetector {
    trees: Vec<IsolationTree>,
    n_trees: usize,
    sample_size: usize,
    contamination: f64,
}

impl AnomalyDetector {
    pub fn new(n_trees: usize, sample_size: usize, contamination: f64) -> Self {
        AnomalyDetector { trees: Vec::new(), n_trees, sample_size, contamination }
    }

    pub fn fit(&mut self, data: &[Vec<f64>]) {
        self.trees.clear();
        let max_depth = (self.sample_size as f64).log2().ceil() as usize;
        let n = data.len();

        for _ in 0..self.n_trees {
            let sample: Vec<Vec<f64>> = (0..self.sample_size.min(n))
                .map(|_| data[rand::random::<usize>() % n].clone())
                .collect();
            self.trees.push(IsolationTree::new(&sample, 0, max_depth));
        }
    }

    pub fn predict(&self, points: &[Vec<f64>]) -> Vec<AnomalyResult> {
        let c_n = IsolationTree::c_factor(self.sample_size);
        let threshold = self.calculate_threshold();

        points.iter().map(|point| {
            let avg_path: f64 = self.trees.iter()
                .map(|tree| tree.path_length(point, 0))
                .sum::<f64>() / self.trees.len() as f64;
            let score = 2.0f64.powf(-avg_path / c_n);
            AnomalyResult {
                is_anomaly: score > threshold,
                score,
                threshold,
                method: "isolation_forest".into(),
                details: {
                    let mut d = HashMap::new();
                    d.insert("avg_path_length".into(), avg_path);
                    d.insert("c_n".into(), c_n);
                    d
                },
            }
        }).collect()
    }

    fn calculate_threshold(&self) -> f64 {
        1.0 - self.contamination
    }

    pub fn zscore_detect(data: &[f64], threshold: f64) -> Vec<AnomalyResult> {
        let n = data.len();
        if n < 2 { return data.iter().map(|_| AnomalyResult {
            is_anomaly: false, score: 0.0, threshold, method: "zscore".into(), details: HashMap::new(),
        }).collect(); }

        let mean: f64 = data.iter().sum::<f64>() / n as f64;
        let variance: f64 = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
        let std_dev = variance.sqrt();

        data.iter().map(|&x| {
            let z = if std_dev > 0.0 { ((x - mean) / std_dev).abs() } else { 0.0 };
            let mut d = HashMap::new();
            d.insert("z_score".into(), z);
            d.insert("mean".into(), mean);
            d.insert("std_dev".into(), std_dev);
            AnomalyResult { is_anomaly: z > threshold, score: z, threshold, method: "zscore".into(), details: d }
        }).collect()
    }

    pub fn iqr_detect(data: &[f64], multiplier: f64) -> Vec<AnomalyResult> {
        let n = data.len();
        if n < 4 { return data.iter().map(|_| AnomalyResult {
            is_anomaly: false, score: 0.0, threshold: multiplier, method: "iqr".into(), details: HashMap::new(),
        }).collect(); }

        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let q1_idx = n / 4;
        let q3_idx = 3 * n / 4;
        let q1 = sorted[q1_idx];
        let q3 = sorted[q3_idx];
        let iqr = q3 - q1;
        let lower = q1 - multiplier * iqr;
        let upper = q3 + multiplier * iqr;

        data.iter().map(|&x| {
            let is_out = x < lower || x > upper;
            let score = if iqr > 0.0 { ((x - q1) / iqr).abs() } else { 0.0 };
            let mut d = HashMap::new();
            d.insert("q1".into(), q1); d.insert("q3".into(), q3);
            d.insert("iqr".into(), iqr); d.insert("lower".into(), lower); d.insert("upper".into(), upper);
            AnomalyResult { is_anomaly: is_out, score, threshold: multiplier, method: "iqr".into(), details: d }
        }).collect()
    }

    pub fn time_series_anomaly(series: &[f64], window: usize, threshold: f64) -> Vec<AnomalyResult> {
        let n = series.len();
        if n < window { return series.iter().map(|_| AnomalyResult {
            is_anomaly: false, score: 0.0, threshold, method: "time_series".into(), details: HashMap::new(),
        }).collect(); }

        let mut results = Vec::with_capacity(n);
        for i in 0..n {
            let start = if i >= window { i - window } else { 0 };
            let end = i;
            let slice = &series[start..end];
            if slice.is_empty() {
                results.push(AnomalyResult { is_anomaly: false, score: 0.0, threshold, method: "time_series".into(), details: HashMap::new() });
                continue;
            }
            let mean = slice.iter().sum::<f64>() / slice.len() as f64;
            let deviation = (series[i] - mean).abs();
            let score = if mean > 0.0 { deviation / mean } else { deviation };
            results.push(AnomalyResult {
                is_anomaly: score > threshold, score, threshold, method: "time_series".into(),
                details: {
                    let mut d = HashMap::new();
                    d.insert("mean".into(), mean);
                    d.insert("deviation".into(), deviation);
                    d
                },
            });
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isolation_forest_fit_predict() {
        let data: Vec<Vec<f64>> = (0..100).map(|i| vec![i as f64, (i % 10) as f64]).collect();
        let mut detector = AnomalyDetector::new(50, 64, 0.1);
        detector.fit(&data);
        let results = detector.predict(&[vec![5.0, 5.0], vec![500.0, 500.0]]);
        assert!(results.len() == 2);
        assert!(results[1].score > results[0].score); // Outlier harusnya skor lebih tinggi
    }

    #[test]
    fn test_zscore_detect() {
        let data = vec![1.0, 2.0, 2.5, 3.0, 100.0];
        let results = AnomalyDetector::zscore_detect(&data, 2.0);
        assert!(results[4].is_anomaly);
        assert!(!results[0].is_anomaly);
    }

    #[test]
    fn test_iqr_detect() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 100.0];
        let results = AnomalyDetector::iqr_detect(&data, 1.5);
        assert!(results[5].is_anomaly);
    }

    #[test]
    fn test_time_series_anomaly() {
        let series: Vec<f64> = (0..50).map(|i| i as f64).chain(std::iter::once(500.0)).collect();
        let results = AnomalyDetector::time_series_anomaly(&series, 10, 0.5);
        assert!(results[50].is_anomaly);
    }
}
