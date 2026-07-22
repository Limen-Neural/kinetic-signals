// SPDX-License-Identifier: MIT OR Apache-2.0

//! Streaming technical indicators for real-valued signals.
//!
//! Provides lightweight, allocation-conscious estimators suitable for
//! high-velocity update loops:
//!
//! - [`EMA`] — exponential moving average
//! - [`SMA`] — fixed-window simple moving average
//! - [`ZScore`] — z-score (standard-score) normalization helper

/// Exponential moving average (EMA) for streaming data.
///
/// Smoothing factor \(\alpha = 2 / (\text{period} + 1)\). The first
/// [`update`](EMA::update) seeds the average; subsequent calls blend the new
/// sample with the previous value.
///
/// # Example
///
/// ```rust
/// use kinetic_signals::EMA;
///
/// let mut ema = EMA::new(9);
/// assert_eq!(ema.update(100.0), 100.0);
/// let next = ema.update(110.0);
/// assert!(next > 100.0 && next < 110.0);
/// ```
#[derive(Debug, Clone)]
pub struct EMA {
    /// Current EMA value.
    pub value: f64,
    /// Smoothing factor \(\alpha \in (0, 1]\).
    pub alpha: f64,
    /// Whether at least one sample has been observed.
    pub initialized: bool,
}

impl EMA {
    /// Create an EMA with the classic period-based \(\alpha\).
    pub fn new(period: usize) -> Self {
        let alpha = 2.0 / (period as f64 + 1.0);
        Self {
            value: 0.0,
            alpha,
            initialized: false,
        }
    }

    /// Incorporate `new_value` and return the updated EMA.
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

/// Z-score tracking helper for signal normalization.
///
/// Holds optional running mean / std-dev state; the primary entry point is the
/// static [`compute`](ZScore::compute) method.
#[derive(Debug, Clone)]
pub struct ZScore {
    /// Reference mean (caller-managed; not updated automatically).
    pub mean: f64,
    /// Reference standard deviation (caller-managed).
    pub std_dev: f64,
}

impl ZScore {
    /// Return \((value - mean) / std_dev\), or `0.0` if `std_dev` is near zero.
    pub fn compute(value: f64, mean: f64, std_dev: f64) -> f64 {
        if std_dev > 1e-12 {
            (value - mean) / std_dev
        } else {
            0.0
        }
    }
}

/// Simple moving average (SMA) over a fixed-capacity window.
///
/// When the window is full, the oldest sample is dropped on each update so
/// memory stays O(capacity).
///
/// # Example
///
/// ```rust
/// use kinetic_signals::SMA;
///
/// let mut sma = SMA::new(3);
/// sma.update(1.0);
/// sma.update(2.0);
/// assert_eq!(sma.update(3.0), 2.0);
/// assert_eq!(sma.update(4.0), 3.0);
/// ```
#[derive(Debug, Clone)]
pub struct SMA {
    /// Samples currently in the window (oldest first).
    pub window: Vec<f64>,
    /// Maximum number of samples retained.
    pub capacity: usize,
    /// Running sum of samples in `window`.
    pub sum: f64,
}

impl SMA {
    /// Create an SMA that retains at most `capacity` samples.
    pub fn new(capacity: usize) -> Self {
        Self {
            window: Vec::with_capacity(capacity),
            capacity,
            sum: 0.0,
        }
    }

    /// Incorporate `new_value` and return the updated window mean.
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
