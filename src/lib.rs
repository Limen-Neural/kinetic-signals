//! # kinetic-signals
//!
//! Streaming feature extraction for high-velocity stochastic signals.
//!
//! A high-performance Rust crate for computing streaming statistics, long-memory estimates,
//! point-process intensity, and return-based surprise metrics.
//!
//! ## Features
//!
//! - **Hurst Exponent** - Detects long-term memory and persistence in time-series data
//! - **Hawkes Process** - Models self-exciting event clusters in point processes
//! - **GBM Surprise** - Detects anomalous return magnitudes using Geometric Brownian Motion
//! - **Volatility** - Real-time variance and standard deviation tracking
//! - **Shannon Entropy** - Measures signal complexity and information density
//! - **Indicators** - Moving averages (EMA, SMA) and Z-score tracking
//!
//! ## Performance (Ryzen 9 9950X)
//!
//! - Hurst (100 samples): ~50μs
//! - Hawkes (10 events): ~5μs
//! - GBM Surprise: ~100ns
//!
//! ## Example
//!
//! ```rust
//! use kinetic_signals::{compute_hurst, compute_gbm_surprise, GBMParams};
//!
//! let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let h_result = compute_hurst(&data);
//!
//! let gbm_params = GBMParams::default();
//! let surprise = compute_gbm_surprise(150.0, 100.0, &gbm_params);
//! ```

pub mod entropy;
pub mod gbm;
pub mod hawkes;
pub mod hurst;
pub mod indicators;
mod real;
pub mod stats;
pub mod volatility;

pub use entropy::{compute_shannon_entropy, EntropyResult};
pub use gbm::{compute_gbm_surprise, compute_gbm_surprise_sequence, detect_anomaly, GBMParams, GBMResult};
pub use hawkes::{compute_hawkes, compute_hawkes_streaming, HawkesParams, HawkesResult};
pub use hurst::{compute_hurst, HurstResult};
pub use indicators::{EMA, SMA, ZScore};
pub use stats::{compute_signal_stats, SignalStats};
pub use volatility::VolEstimator;

pub mod prelude {
    pub use crate::entropy::*;
    pub use crate::gbm::*;
    pub use crate::hawkes::*;
    pub use crate::hurst::*;
    pub use crate::indicators::*;
    pub use crate::stats::*;
    pub use crate::volatility::*;
}
