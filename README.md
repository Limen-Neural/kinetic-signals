# kinetic-signals

Streaming feature extraction for high-velocity stochastic signals.

A high-performance Rust crate for computing streaming signal statistics, point-process intensity features, and anomaly metrics on stochastic time-series.

## Features

- **Zero dependencies** - No external crates required
- **Hurst Exponent** - Detects long-term memory and persistence in time-series data
- **Hawkes Process** - Models self-exciting event clusters in point-process streams
- **GBM Surprise** - Detects anomalous return magnitudes with Geometric Brownian Motion
- **Volatility** - Real-time variance and standard deviation tracking
- **Shannon Entropy** - Measures signal complexity and information density
- **Indicators** - Moving averages (EMA, SMA) and Z-score tracking
- **Signal Stats** - High-order moments (Skewness, Kurtosis)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
kinetic-signals = { git = "https://github.com/Raul-BioMEMS/kinetic-signals" }
```

## Usage

```rust
use kinetic_signals::{
    compute_hurst, compute_hawkes, compute_gbm_surprise, detect_anomaly,
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

### Demo

Run the included demo:

```bash
cargo run --example demo
```

### Numeric types

Most APIs use `f64`. `compute_hurst` and GBM helpers are generic and support `f32` and `f64`.

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
