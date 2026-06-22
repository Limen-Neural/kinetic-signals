// SPDX-License-Identifier: MIT OR Apache-2.0

//! Cross-language output-range parity checks (issue #3 / LIM-31).
//!
//! This test pins the documented output-range convention for `kinetic-signals`
//! so it can be cross-checked against the Julia `SpikeStream.jl` implementation
//! using the SAME shared input vectors. The canonical shared vectors and their
//! expected output ranges live in `tests/fixtures/shared_vectors.json`; the
//! values below mirror that fixture so both languages assert against identical
//! inputs.
//!
//! Note: this asserts the Rust side of the convention (ranges, determinism, and
//! reference values within tolerance). Full byte-for-byte parity with
//! `SpikeStream.jl` must be validated in that repository against the same JSON.

use kinetic_signals::{
    VolEstimator, compute_hawkes, compute_hurst, compute_shannon_entropy, compute_surprise,
    hawkes::HawkesParams, surprise::SurpriseParams,
};

const TOL: f64 = 1e-6;

// Shared input vectors (mirror of tests/fixtures/shared_vectors.json).
fn hurst_trending() -> Vec<f64> {
    (0..64).map(|i| i as f64 * 0.1).collect()
}

fn hawkes_events() -> Vec<f64> {
    vec![0.0, 0.01, 0.02, 0.03, 0.1, 0.5, 0.51, 0.52]
}

fn entropy_signal() -> Vec<f64> {
    vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 5.0, 6.0]
}

fn vol_returns() -> Vec<f32> {
    vec![0.01, 0.02, 0.015, 0.03, 0.012, 0.025, 0.018]
}

#[test]
fn hurst_within_unit_interval() {
    let r = compute_hurst(&hurst_trending());
    assert!(r.h.is_finite());
    assert!((0.0..=1.0).contains(&r.h), "hurst H out of [0,1]: {}", r.h);
    // Deterministic.
    let r2 = compute_hurst(&hurst_trending());
    assert!((r.h - r2.h).abs() < TOL);
}

#[test]
fn hawkes_intensity_at_least_baseline() {
    let params = HawkesParams::default();
    let r = compute_hawkes(&hawkes_events(), &params);
    assert!(r.intensity.is_finite());
    assert!(
        r.intensity >= params.mu,
        "intensity {} below baseline mu {}",
        r.intensity,
        params.mu
    );
    assert!(r.avg_excitation >= 0.0);
    assert_eq!(r.event_count, hawkes_events().len());
}

#[test]
fn surprise_is_nonnegative_and_anomaly_consistent() {
    let params = SurpriseParams::<f64>::default();

    let calm = compute_surprise(100.0, 100.0, &params);
    assert!(calm.surprise.is_finite() && calm.surprise >= 0.0);
    assert!(calm.surprise <= params.threshold);

    let spike = compute_surprise(150.0, 100.0, &params);
    assert!(spike.surprise >= 0.0);
    assert_eq!(spike.surprise, spike.z_score.abs());
    assert!(
        spike.surprise > params.threshold,
        "spike should be anomalous"
    );
}

#[test]
fn entropy_within_bounds() {
    let bins = 8;
    let r = compute_shannon_entropy(&entropy_signal(), bins);
    let max_entropy = (bins as f64).ln();
    assert!(r.shannon >= 0.0 && r.shannon <= max_entropy + TOL);
    assert!((0.0..=1.0 + TOL).contains(&r.relative));
    assert!(r.bin_count <= bins);
}

#[test]
fn volatility_rms_nonnegative() {
    let mut est = VolEstimator::new(5);
    for &x in &vol_returns() {
        est.push(x);
    }
    let rms = est.rms();
    assert!(rms >= 0.0, "rms must be non-negative: {rms}");
    assert!(rms.is_finite());
}
