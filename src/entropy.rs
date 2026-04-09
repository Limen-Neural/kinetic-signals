use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyResult {
    pub shannon: f64,
    pub relative: f64, // normalized to [0, 1]
    pub bin_count: usize,
}

/// Compute Shannon entropy of a signal using histogram discretization
pub fn compute_shannon_entropy(data: &[f64], bins: usize) -> EntropyResult {
    if data.len() < 2 || bins == 0 {
        return EntropyResult {
            shannon: 0.0,
            relative: 0.0,
            bin_count: 0,
        };
    }

    let min = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let range = max - min;

    if range == 0.0 {
        return EntropyResult {
            shannon: 0.0,
            relative: 0.0,
            bin_count: 1,
        };
    }

    let mut histogram = vec![0usize; bins];
    for &x in data {
        let bin = (((x - min) / range) * (bins as f64 - 1e-9)).floor() as usize;
        let bin = bin.min(bins - 1);
        histogram[bin] += 1;
    }

    let n = data.len() as f64;
    let mut shannon = 0.0;
    let mut actual_bins = 0;

    for &count in &histogram {
        if count > 0 {
            let p = count as f64 / n;
            shannon -= p * p.ln();
            actual_bins += 1;
        }
    }

    let max_entropy = (bins as f64).ln();
    let relative = if max_entropy > 0.0 {
        shannon / max_entropy
    } else {
        0.0
    };

    EntropyResult {
        shannon,
        relative,
        bin_count: actual_bins,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_uniform() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let res = compute_shannon_entropy(&data, 4);
        assert!(res.shannon > 0.0);
        assert_eq!(res.bin_count, 4);
    }

    #[test]
    fn test_entropy_constant() {
        let data = vec![1.0, 1.0, 1.0, 1.0];
        let res = compute_shannon_entropy(&data, 4);
        assert_eq!(res.shannon, 0.0);
        assert_eq!(res.bin_count, 1);
    }
}
