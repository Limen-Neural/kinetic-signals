use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HurstResult {
    pub h: f64,
    pub is_persistent: bool,
    pub is_antipersistent: bool,
}

pub fn compute_hurst(data: &[f64]) -> HurstResult {
    let n = data.len();
    if n < 32 {
        return HurstResult {
            h: 0.5,
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
        let mut rs_sums = 0.0;
        let mut count = 0;

        // Efficiently compute R/S for non-overlapping chunks
        for i in (0..=(n - tau)).step_by(tau) {
            let chunk = &data[i..i + tau];
            let mean = chunk.iter().sum::<f64>() / tau as f64;

            let mut cumdev = 0.0;
            let mut max_dev = 0.0f64;
            let mut min_dev = 0.0f64;
            let mut sq_diff_sum = 0.0;

            for &x in chunk {
                let diff = x - mean;
                cumdev += diff;
                max_dev = max_dev.max(cumdev);
                min_dev = min_dev.min(cumdev);
                sq_diff_sum += diff * diff;
            }

            let std_dev = (sq_diff_sum / tau as f64).sqrt();
            if std_dev > 1e-12 {
                rs_sums += (max_dev - min_dev) / std_dev;
                count += 1;
            }
        }

        if count > 0 {
            let rs_avg = rs_sums / count as f64;
            if rs_avg > 0.0 {
                log_rs.push(rs_avg.ln());
                log_n.push((tau as f64).ln());
            }
        }
    }

    // Linear regression on log-log plot to find the Hurst exponent (slope)
    let h = if log_n.len() < 2 {
        0.5
    } else {
        let n_mean = log_n.iter().sum::<f64>() / log_n.len() as f64;
        let rs_mean = log_rs.iter().sum::<f64>() / log_rs.len() as f64;

        let num = log_n
            .iter()
            .zip(log_rs.iter())
            .map(|(&x, &y)| (x - n_mean) * (y - rs_mean))
            .sum::<f64>();
        let den = log_n.iter().map(|&x| (x - n_mean).powi(2)).sum::<f64>();

        if den.abs() < 1e-12 {
            0.5
        } else {
            num / den
        }
    };

    let h = h.clamp(0.0, 1.0);
    HurstResult {
        h,
        is_persistent: h > 0.52, // Small buffer around 0.5
        is_antipersistent: h < 0.48,
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
}
