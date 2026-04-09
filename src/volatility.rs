use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolatilityResult {
    pub std_dev: f64,
    pub variance: f64,
    pub mean: f64,
}

pub fn compute_volatility(data: &[f64]) -> VolatilityResult {
    if data.len() < 2 {
        return VolatilityResult {
            std_dev: 0.0,
            variance: 0.0,
            mean: data.first().copied().unwrap_or(0.0),
        };
    }

    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;
    let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n;

    VolatilityResult {
        std_dev: variance.sqrt(),
        variance,
        mean,
    }
}

/// Moving Volatility using Welford's algorithm for streaming data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovingVolatility {
    pub count: usize,
    pub mean: f64,
    pub m2: f64,
}

impl MovingVolatility {
    pub fn new() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }

    pub fn update(&mut self, new_value: f64) {
        self.count += 1;
        let delta = new_value - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = new_value - self.mean;
        self.m2 += delta * delta2;
    }

    pub fn variance(&self) -> f64 {
        if self.count < 2 {
            0.0
        } else {
            self.m2 / self.count as f64
        }
    }

    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }

    pub fn result(&self) -> VolatilityResult {
        VolatilityResult {
            std_dev: self.std_dev(),
            variance: self.variance(),
            mean: self.mean,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volatility_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = compute_volatility(&data);
        assert_eq!(result.mean, 3.0);
        assert!(result.std_dev > 0.0);
    }

    #[test]
    fn test_moving_volatility() {
        let mut mv = MovingVolatility::new();
        mv.update(1.0);
        mv.update(2.0);
        mv.update(3.0);
        let res = mv.result();
        assert_eq!(res.mean, 2.0);
        assert!(res.std_dev > 0.0);
    }
}

impl Default for MovingVolatility {
    fn default() -> Self {
        Self::new()
    }
}
