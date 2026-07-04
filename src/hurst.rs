// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::real::Real;

#[derive(Debug, Clone)]
pub struct HurstResult<T = f64> {
    pub h: T,
    pub is_persistent: bool,
    pub is_antipersistent: bool,
}

fn c<T: Real>(value: f64) -> T {
    T::from_f64(value)
}

pub fn compute_hurst<T>(data: &[T]) -> HurstResult<T>
where
    T: Real,
{
    let n = data.len();
    if n < 32 {
        return HurstResult {
            h: c(0.5),
            is_persistent: false,
            is_antipersistent: false,
        };
    }

    // Use logarithmically spaced tau values for better scale coverage
    let mut tau_values = Vec::new();
    let mut current_tau = 8usize;
    while current_tau <= n / 2 {
        tau_values.push(current_tau);
        current_tau = (current_tau as f64 * 1.4).ceil() as usize;
        if tau_values.len() >= 30 {
            break;
        }
    }

    let mut log_rs = Vec::with_capacity(tau_values.len());
    let mut log_n = Vec::with_capacity(tau_values.len());

    for &tau in &tau_values {
        let mut rs_sums = T::zero();
        let mut count = 0;

        // Efficiently compute R/S for non-overlapping chunks
        for i in (0..=(n - tau)).step_by(tau) {
            let chunk: &[T] = &data[i..i + tau];
            let mean: T =
                chunk.iter().copied().fold(T::zero(), |acc, x| acc + x) / T::from_usize(tau);

            let mut cumdev = T::zero();
            let mut max_dev = T::zero();
            let mut min_dev = T::zero();
            let mut sq_diff_sum = T::zero();

            for &x in chunk {
                let diff = x - mean;
                cumdev = cumdev + diff;
                max_dev = max_dev.max(cumdev);
                min_dev = min_dev.min(cumdev);
                sq_diff_sum = sq_diff_sum + diff * diff;
            }

            let std_dev = (sq_diff_sum / T::from_usize(tau)).sqrt();
            if std_dev > c(1e-12) {
                rs_sums = rs_sums + (max_dev - min_dev) / std_dev;
                count += 1;
            }
        }

        if count > 0 {
            let rs_avg = rs_sums / T::from_usize(count);
            if rs_avg > T::zero() {
                log_rs.push(rs_avg.ln());
                log_n.push(T::from_usize(tau).ln());
            }
        }
    }

    // Linear regression on log-log plot to find the Hurst exponent (slope)
    let h = if log_n.len() < 2 {
        c(0.5)
    } else {
        let n_mean =
            log_n.iter().copied().fold(T::zero(), |acc, x| acc + x) / T::from_usize(log_n.len());
        let rs_mean =
            log_rs.iter().copied().fold(T::zero(), |acc, x| acc + x) / T::from_usize(log_rs.len());

        let num = log_n
            .iter()
            .zip(log_rs.iter())
            .fold(T::zero(), |acc, (&x, &y)| {
                acc + (x - n_mean) * (y - rs_mean)
            });
        let den = log_n
            .iter()
            .fold(T::zero(), |acc, &x| acc + (x - n_mean).powi(2));

        if den.abs() < c(1e-12) {
            c(0.5)
        } else {
            num / den
        }
    };

    let h = h.max(T::zero()).min(T::one());
    HurstResult {
        h,
        is_persistent: h > c(0.52), // Small buffer around 0.5
        is_antipersistent: h < c(0.48),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hurst_basic() {
        let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = compute_hurst(&data);
        assert!(result.h >= 0.0 && result.h <= 1.0);
    }

    #[test]
    fn test_hurst_f32_support() {
        let data: Vec<f32> = (0..64).map(|i| i as f32 * 0.1).collect();
        let result = compute_hurst(&data);
        assert!(result.h >= 0.0_f32 && result.h <= 1.0_f32);
    }
}
