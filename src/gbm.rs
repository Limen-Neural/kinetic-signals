use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GBMResult {
    pub surprise: f64,
    pub log_return: f64,
    pub expected_return: f64,
    pub z_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GBMParams {
    pub mu: f64,
    pub sigma: f64,
    pub dt: f64,
    pub threshold: f64,
}

impl Default for GBMParams {
    fn default() -> Self {
        GBMParams {
            mu: 0.0,
            sigma: 0.1,
            dt: 0.001,
            threshold: 3.0,
        }
    }
}

pub fn compute_gbm_surprise(
    current_value: f64,
    previous_value: f64,
    params: &GBMParams,
) -> GBMResult {
    if previous_value <= 0.0 || current_value <= 0.0 {
        return GBMResult {
            surprise: 0.0,
            log_return: 0.0,
            expected_return: params.mu * params.dt,
            z_score: 0.0,
        };
    }

    let log_return = (current_value / previous_value).ln();

    let expected_return = params.mu * params.dt;

    let std_dev = params.sigma * params.dt.sqrt();

    let z_score = if std_dev > 0.0 {
        (log_return - expected_return) / std_dev
    } else {
        0.0
    };

    let surprise = z_score.abs();

    GBMResult {
        surprise,
        log_return,
        expected_return,
        z_score,
    }
}

pub fn compute_gbm_surprise_sequence(values: &[f64], params: &GBMParams) -> Vec<GBMResult> {
    if values.len() < 2 {
        return Vec::new();
    }

    let mut results = Vec::with_capacity(values.len() - 1);

    for i in 1..values.len() {
        let result = compute_gbm_surprise(values[i], values[i - 1], params);
        results.push(result);
    }

    results
}

pub fn detect_anomaly(gbm_result: &GBMResult, params: &GBMParams) -> bool {
    gbm_result.surprise > params.threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gbm_normal() {
        let params = GBMParams::default();
        let result = compute_gbm_surprise(100.0, 99.0, &params);
        assert!(result.log_return > 0.0);
    }

    #[test]
    fn test_gbm_spike() {
        let params = GBMParams::default();
        let result = compute_gbm_surprise(200.0, 100.0, &params);
        assert!(result.surprise > 1.0);
    }

    #[test]
    fn test_gbm_zero_protection() {
        let params = GBMParams::default();
        let result = compute_gbm_surprise(0.0, 100.0, &params);
        assert_eq!(result.surprise, 0.0);
    }
}
