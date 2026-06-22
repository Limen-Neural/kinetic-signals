// SPDX-License-Identifier: MIT OR Apache-2.0

#[derive(Debug, Clone)]
pub struct SignalStats {
    pub mean: f64,
    pub variance: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    pub count: usize,
}

/// Compute high-order moments for a signal using a single-pass algorithm
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
}
