// SPDX-License-Identifier: MIT OR Apache-2.0

/// Exponential Moving Average (EMA) for streaming data
#[derive(Debug, Clone)]
pub struct EMA {
    pub value: f64,
    pub alpha: f64,
    pub initialized: bool,
}

impl EMA {
    pub fn new(period: usize) -> Self {
        let alpha = 2.0 / (period as f64 + 1.0);
        Self {
            value: 0.0,
            alpha,
            initialized: false,
        }
    }

    pub fn update(&mut self, new_value: f64) -> f64 {
        if !self.initialized {
            self.value = new_value;
            self.initialized = true;
        } else {
            self.value = self.alpha * new_value + (1.0 - self.alpha) * self.value;
        }
        self.value
    }
}

/// Z-Score tracking for signal normalization
#[derive(Debug, Clone)]
pub struct ZScore {
    pub mean: f64,
    pub std_dev: f64,
}

impl ZScore {
    pub fn compute(value: f64, mean: f64, std_dev: f64) -> f64 {
        if std_dev > 1e-12 {
            (value - mean) / std_dev
        } else {
            0.0
        }
    }
}

/// Simple Moving Average (SMA) with a fixed window
#[derive(Debug, Clone)]
pub struct SMA {
    pub window: Vec<f64>,
    pub capacity: usize,
    pub sum: f64,
}

impl SMA {
    pub fn new(capacity: usize) -> Self {
        Self {
            window: Vec::with_capacity(capacity),
            capacity,
            sum: 0.0,
        }
    }

    pub fn update(&mut self, new_value: f64) -> f64 {
        if self.window.len() == self.capacity {
            self.sum -= self.window.remove(0);
        }
        self.window.push(new_value);
        self.sum += new_value;
        self.sum / self.window.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ema() {
        let mut ema = EMA::new(9);
        assert_eq!(ema.update(100.0), 100.0);
        let next = ema.update(110.0);
        assert!(next > 100.0 && next < 110.0);
    }

    #[test]
    fn test_sma() {
        let mut sma = SMA::new(3);
        sma.update(1.0);
        sma.update(2.0);
        assert_eq!(sma.update(3.0), 2.0);
        assert_eq!(sma.update(4.0), 3.0);
    }
}
