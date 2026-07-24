// SPDX-License-Identifier: MIT OR Apache-2.0

//! Higher-order moments of a real-valued signal.
//!
//! [`compute_signal_stats`] computes mean, variance, skewness, and excess
//! kurtosis in a single pass after the mean is known (two passes total over
//! the slice). Suitable for batch feature extraction; for streaming variance
//! prefer [`crate::VolEstimator`].

/// Central moments and shape descriptors of a signal sample.
#[derive(Debug, Clone)]
pub struct SignalStats {
    /// Arithmetic mean.
    pub mean: f64,
    /// Population variance (\( m_2 / n \), not Bessel-corrected).
    pub variance: f64,
    /// Sample skewness (\( m_3 / \sigma^3 \)); `0.0` when variance is near zero.
    pub skewness: f64,
    /// Excess kurtosis (\( m_4 / \sigma^4 - 3 \)); `0.0` for a Gaussian or degenerate series.
    pub kurtosis: f64,
    /// Number of samples used.
    pub count: usize,
}

/// Compute high-order moments for a signal using a single-pass algorithm
/// (after the mean is computed).
///
/// Returns an all-zero result for an empty slice.
///
/// # Example
///
/// ```rust
/// use kinetic_signals::compute_signal_stats;
///
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let stats = compute_signal_stats(&data);
/// assert_eq!(stats.mean, 3.0);
/// assert_eq!(stats.count, 5);
/// assert!(stats.variance > 0.0);
/// ```
pub fn compute_signal_stats(data: &[f64]) -> SignalStats {
    let n = data.len();
    if n == 0 {
        return SignalStats {
            mean: 0.0,
            variance: 0.0,
            skewness: 0.0,
            kurtosis: 0.0,
            count: 0,
        };
    }

    let n_f = n as f64;
    let mean = data.iter().sum::<f64>() / n_f;

    let mut m2 = 0.0;
    let mut m3 = 0.0;
    let mut m4 = 0.0;

    for &x in data {
        let diff = x - mean;
        let d2 = diff * diff;
        m2 += d2;
        m3 += d2 * diff;
        m4 += d2 * d2;
    }

    let var = m2 / n_f;
    let std = var.sqrt();

    let skew = if std > 1e-12 {
        (m3 / n_f) / (std * var)
    } else {
        0.0
    };

    let kurt = if var > 1e-12 {
        (m4 / n_f) / (var * var) - 3.0
    } else {
        0.0
    };

    SignalStats {
        mean,
        variance: var,
        skewness: skew,
        kurtosis: kurt,
        count: n,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_stats() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = compute_signal_stats(&data);
        assert_eq!(stats.mean, 3.0);
        assert!(stats.variance > 0.0);
        // Normal-ish distribution should have low skewness
        assert!(stats.skewness.abs() < 0.1);
    }

    #[test]
    fn test_signal_stats_empty() {
        let stats = compute_signal_stats(&[]);
        assert_eq!(stats.count, 0);
        assert_eq!(stats.mean, 0.0);
        assert_eq!(stats.variance, 0.0);
        assert_eq!(stats.skewness, 0.0);
        assert_eq!(stats.kurtosis, 0.0);
    }

    #[test]
    fn test_signal_stats_single_element() {
        let stats = compute_signal_stats(&[7.5]);
        assert_eq!(stats.count, 1);
        assert_eq!(stats.mean, 7.5);
        assert_eq!(stats.variance, 0.0);
        assert_eq!(stats.skewness, 0.0);
        assert_eq!(stats.kurtosis, 0.0);
    }

    #[test]
    fn test_signal_stats_constant_values() {
        let data = vec![3.0, 3.0, 3.0, 3.0, 3.0];
        let stats = compute_signal_stats(&data);
        assert_eq!(stats.count, 5);
        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.variance, 0.0);
        assert_eq!(stats.skewness, 0.0);
        assert_eq!(stats.kurtosis, 0.0);
    }
}
