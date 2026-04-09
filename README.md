# spikenaut-signals

Streaming time-series feature extraction for Spikenaut SNNs (Spiking Neural Networks).

A high-performance, zero-overhead Rust crate for computing neuromorphic signal primitives used in the Spikenaut architecture. Designed for sub-millisecond execution on AMD Ryzen 9 9950X.

## Provenance

Extracted from Eagle-Lander, the author's own private neuromorphic GPU supervisor repository (closed-source). The Hurst, Hawkes, and GBM modules were used in production to extract real-time features from GPU telemetry and HFT data streams for SNN input.

## Features

- **Hurst Exponent** - Detects long-term memory and persistence in time-series data
- **Hawkes Process** - Models self-exciting event clusters (PCIe floods, spike bursts)
- **GBM Surprise** - Detects anomalous power transients using Geometric Brownian Motion
- **Volatility** - Real-time variance and standard deviation tracking
- **Shannon Entropy** - Measures signal complexity and information density
- **Indicators** - Moving averages (EMA, SMA) and Z-score tracking
- **Signal Stats** - High-order moments (Skewness, Kurtosis)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
spikenaut-signals = { path = "../spikenaut-signals" }
```

## Usage

```rust
use spikenaut_signals::{
    compute_hurst, compute_hawkes, compute_gbm_surprise,
    hawkes::HawkesParams, gbm::GBMParams,
};

// Hurst Exponent - detect trending vs random behavior
let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let result = compute_hurst(&data);
println!("H = {:.3}, persistent = {}", result.h, result.is_persistent);

// Hawkes Process - model event clustering
let params = HawkesParams::default();
let events = vec![0.0, 0.01, 0.02, 0.1, 0.5];
let result = compute_hawkes(&events, &params);
println!("Intensity = {:.3}", result.intensity);

// GBM Surprise - detect power spikes
let params = GBMParams::default();
let surprise = compute_gbm_surprise(150.0, 100.0, &params);
if detect_anomaly(&surprise, &params) {
    println!("ANOMALY DETECTED! z = {:.2}", surprise.z_score);
}
```

## Performance

Built with aggressive optimizations for real-time inference:

Typical execution times (Ryzen 9 9950X):
- Hurst (100 samples): ~50μs
- Hawkes (10 events): ~5μs
- GBM Surprise: ~100ns

## License

GPL-3.0

## Authors

Raul Montoya Cardenas
