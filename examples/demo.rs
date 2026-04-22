use kinetic_signals::{
    compute_gbm_surprise, compute_hawkes, compute_hurst, compute_shannon_entropy,
    gbm::GBMParams, hawkes::HawkesParams, VolEstimator,
};

fn lcg_next(state: &mut u64) -> u64 {
    // Numerical Recipes LCG constants (good enough for a demo; not crypto-secure)
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
    *state
}

fn pseudo_random_f64(state: &mut u64) -> f64 {
    // Map top 53 bits to [0, 1)
    let x = lcg_next(state) >> 11;
    (x as f64) / ((1u64 << 53) as f64)
}

fn main() {
    println!("=== Kinetic Signals Demo v0.2.0 ===\n");

    demo_hurst();
    demo_hawkes();
    demo_gbm();
    demo_volatility();
    demo_entropy();
}

fn demo_hurst() {
    println!("--- Hurst Exponent (Memory/Persistence) ---");

    let persistent: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
    let result = compute_hurst(&persistent);
    println!(
        "Trending data H={:.3} (persistent={})",
        result.h, result.is_persistent
    );

    let mut rng = 0x1234_5678_9abc_def0_u64;
    let random: Vec<f64> = (0..100).map(|_| pseudo_random_f64(&mut rng) - 0.5).collect();
    let result = compute_hurst(&random);
    println!(
        "Random data H={:.3} (anti={})",
        result.h, result.is_antipersistent
    );
    println!();
}

fn demo_hawkes() {
    println!("--- Hawkes Process (Event Clustering) ---");

    let params = HawkesParams {
        mu: 0.1,
        alpha: 0.8,
        beta: 2.0,
        dt: 0.001,
    };

    let burst_events: Vec<f64> = vec![0.0, 0.01, 0.02, 0.03, 0.1, 0.5, 0.51, 0.52];
    let result = compute_hawkes(&burst_events, &params);
    println!(
        "Dense burst: intensity={:.3}, events={}, avg_excitation={:.3}",
        result.intensity, result.event_count, result.avg_excitation
    );

    let sparse_events: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0];
    let result = compute_hawkes(&sparse_events, &params);
    println!(
        "Sparse: intensity={:.3}, events={}",
        result.intensity, result.event_count
    );
    println!();
}

fn demo_gbm() {
    println!("--- GBM Surprise (Return Transients) ---");

    let params = GBMParams {
        mu: 0.0,
        sigma: 0.15,
        dt: 0.001,
        threshold: 3.0,
    };

    let normal = compute_gbm_surprise(100.5, 100.0, &params);
    println!(
        "Normal: surprise={:.3}, z={:.2}",
        normal.surprise, normal.z_score
    );

    let spike = compute_gbm_surprise(150.0, 100.0, &params);
    println!(
        "SPIKE: surprise={:.3}, z={:.2} (ANOMALY={})",
        spike.surprise,
        spike.z_score,
        spike.surprise > params.threshold
    );

    let crash = compute_gbm_surprise(50.0, 100.0, &params);
    println!(
        "CRASH: surprise={:.3}, z={:.2} (ANOMALY={})",
        crash.surprise,
        crash.z_score,
        crash.surprise > params.threshold
    );
    println!();
}

fn demo_volatility() {
    println!("--- Volatility (Signal Power) ---");

    let abs_log_returns = vec![0.01_f32, 0.02, 0.015, 0.03, 0.012, 0.025, 0.018];

    let mut estimator = VolEstimator::new(5);
    for &r in &abs_log_returns {
        estimator.push(r);
    }

    println!(
        "Rolling RMS volatility={:.4} (window={}, samples={})",
        estimator.rms(),
        5,
        estimator.len()
    );
    println!();
}

fn demo_entropy() {
    println!("--- Shannon Entropy (Complexity) ---");

    let low_entropy = vec![1.0, 1.0, 1.1, 1.0, 0.9, 1.0];
    let res1 = compute_shannon_entropy(&low_entropy, 10);
    println!(
        "Low complexity: H={:.3}, relative={:.3}, bins={}",
        res1.shannon, res1.relative, res1.bin_count
    );

    let mut rng = 0x0bad_f00d_dead_beef_u64;
    let high_entropy: Vec<f64> = (0..100).map(|_| pseudo_random_f64(&mut rng)).collect();
    let res2 = compute_shannon_entropy(&high_entropy, 10);
    println!(
        "High complexity: H={:.3}, relative={:.3}, bins={}",
        res2.shannon, res2.relative, res2.bin_count
    );
    println!();
}
