//! Domain-agnostic surprise detection.
//!
//! Computes a normalized "surprise" score for a transition between two
//! consecutive positive samples of a stochastic signal. The score is the
//! absolute z-score of the observed log-ratio relative to an expected drift,
//! scaled by the per-step standard deviation.
//!
//! This is a generic signal-processing primitive: it makes no financial-domain
//! assumptions. It can be applied to any strictly positive signal (sensor
//! magnitudes, firing rates, power readings, asset prices, etc.).

use crate::real::Real;

/// Result of a single surprise computation.
#[derive(Debug, Clone)]
pub struct SurpriseResult<T = f64> {
    /// Absolute z-score of the observed transition (always `>= 0`).
    pub surprise: T,
    /// Natural log-ratio of `current / previous`.
    pub log_return: T,
    /// Expected drift over one step (`mu * dt`).
    pub expected_return: T,
    /// Signed z-score of the observed transition.
    pub z_score: T,
}

/// Parameters controlling surprise detection.
#[derive(Debug, Clone)]
pub struct SurpriseParams<T = f64> {
    /// Expected drift rate.
    pub mu: T,
    /// Per-unit-time volatility.
    pub sigma: T,
    /// Time step between samples.
    pub dt: T,
    /// Absolute z-score above which a transition is flagged anomalous.
    pub threshold: T,
}

impl<T> Default for SurpriseParams<T>
where
    T: Real,
{
    fn default() -> Self {
        SurpriseParams {
            mu: T::zero(),
            sigma: T::from_f64(0.1),
            dt: T::from_f64(0.001),
            threshold: T::from_f64(3.0),
        }
    }
}

/// Compute the surprise score for a single transition.
///
/// Returns a zeroed result (no surprise) if either value is non-positive,
/// since the log-ratio is undefined for non-positive inputs.
pub fn compute_surprise<T>(
    current_value: T,
    previous_value: T,
    params: &SurpriseParams<T>,
) -> SurpriseResult<T>
where
    T: Real,
{
    if previous_value <= T::zero() || current_value <= T::zero() {
        return SurpriseResult {
            surprise: T::zero(),
            log_return: T::zero(),
            expected_return: params.mu * params.dt,
            z_score: T::zero(),
        };
    }

    let log_return = (current_value / previous_value).ln();

    let expected_return = params.mu * params.dt;

    let std_dev = params.sigma * params.dt.sqrt();

    let z_score = if std_dev > T::zero() {
        (log_return - expected_return) / std_dev
    } else {
        T::zero()
    };

    let surprise = z_score.abs();

    SurpriseResult {
        surprise,
        log_return,
        expected_return,
        z_score,
    }
}

/// Compute surprise scores for every consecutive transition in `values`.
pub fn compute_surprise_sequence<T>(
    values: &[T],
    params: &SurpriseParams<T>,
) -> Vec<SurpriseResult<T>>
where
    T: Real,
{
    if values.len() < 2 {
        return Vec::new();
    }

    let mut results = Vec::with_capacity(values.len() - 1);

    for i in 1..values.len() {
        let result = compute_surprise(values[i], values[i - 1], params);
        results.push(result);
    }

    results
}

/// Return `true` if the result's surprise exceeds the configured threshold.
pub fn detect_anomaly<T>(result: &SurpriseResult<T>, params: &SurpriseParams<T>) -> bool
where
    T: Real,
{
    result.surprise > params.threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surprise_normal() {
        let params = SurpriseParams::default();
        let result = compute_surprise(100.0, 99.0, &params);
        assert!(result.log_return > 0.0_f64);
    }

    #[test]
    fn test_surprise_spike() {
        let params = SurpriseParams::default();
        let result = compute_surprise(200.0, 100.0, &params);
        assert!(result.surprise > 1.0_f64);
    }

    #[test]
    fn test_surprise_zero_protection() {
        let params = SurpriseParams::default();
        let result = compute_surprise(0.0, 100.0, &params);
        assert_eq!(result.surprise, 0.0_f64);
    }

    #[test]
    fn test_surprise_f32_support() {
        let params = SurpriseParams::<f32>::default();
        let result = compute_surprise(100.5_f32, 100.0_f32, &params);
        assert!(result.surprise >= 0.0_f32);
    }
}
