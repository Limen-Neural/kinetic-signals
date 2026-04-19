use crate::real::Real;

#[derive(Debug, Clone)]
pub struct GBMResult<T = f64> {
    pub surprise: T,
    pub log_return: T,
    pub expected_return: T,
    pub z_score: T,
}

#[derive(Debug, Clone)]
pub struct GBMParams<T = f64> {
    pub mu: T,
    pub sigma: T,
    pub dt: T,
    pub threshold: T,
}

impl<T> Default for GBMParams<T>
where
    T: Real,
{
    fn default() -> Self {
        GBMParams {
            mu: T::zero(),
            sigma: T::from_f64(0.1),
            dt: T::from_f64(0.001),
            threshold: T::from_f64(3.0),
        }
    }
}

pub fn compute_gbm_surprise<T>(
    current_value: T,
    previous_value: T,
    params: &GBMParams<T>,
) -> GBMResult<T>
where
    T: Real,
{
    if previous_value <= T::zero() || current_value <= T::zero() {
        return GBMResult {
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

    GBMResult {
        surprise,
        log_return,
        expected_return,
        z_score,
    }
}

pub fn compute_gbm_surprise_sequence<T>(values: &[T], params: &GBMParams<T>) -> Vec<GBMResult<T>>
where
    T: Real,
{
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

pub fn detect_anomaly<T>(gbm_result: &GBMResult<T>, params: &GBMParams<T>) -> bool
where
    T: Real,
{
    gbm_result.surprise > params.threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gbm_normal() {
        let params = GBMParams::default();
        let result = compute_gbm_surprise(100.0, 99.0, &params);
        assert!(result.log_return > 0.0_f64);
    }

    #[test]
    fn test_gbm_spike() {
        let params = GBMParams::default();
        let result = compute_gbm_surprise(200.0, 100.0, &params);
        assert!(result.surprise > 1.0_f64);
    }

    #[test]
    fn test_gbm_zero_protection() {
        let params = GBMParams::default();
        let result = compute_gbm_surprise(0.0, 100.0, &params);
        assert_eq!(result.surprise, 0.0_f64);
    }

    #[test]
    fn test_gbm_f32_support() {
        let params = GBMParams::<f32>::default();
        let result = compute_gbm_surprise(100.5_f32, 100.0_f32, &params);
        assert!(result.surprise >= 0.0_f32);
    }
}
