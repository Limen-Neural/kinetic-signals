use spikenaut_signals::{
    compute_gbm_surprise, compute_hawkes, compute_hurst, compute_shannon_entropy,
    compute_volatility, gbm::GBMParams, hawkes::HawkesParams, volatility::MovingVolatility,
};

fn main() {
    println!("=== Spikenaut Signals Demo v0.2.0 ===\n");

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

    let random: Vec<f64> = (0..100).map(|_| rand::random::<f64>() - 0.5).collect();
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
        "PCIe burst: intensity={:.3}, events={}, avg_excitation={:.3}",
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
    println!("--- GBM Surprise (Power Transients) ---");

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

    let data = vec![1.0, 1.1, 0.9, 1.2, 0.8, 1.5, 0.5];
    let result = compute_volatility(&data);
    println!(
        "Batch: mean={:.3}, std_dev={:.3}, var={:.3}",
        result.mean, result.std_dev, result.variance
    );

    let mut mv = MovingVolatility::new();
    for &val in &data {
        mv.update(val);
    }
    let m_res = mv.result();
    println!(
        "Streaming: mean={:.3}, std_dev={:.3} (count={})",
        m_res.mean, m_res.std_dev, mv.count
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

    let high_entropy: Vec<f64> = (0..100).map(|_| rand::random::<f64>()).collect();
    let res2 = compute_shannon_entropy(&high_entropy, 10);
    println!(
        "High complexity: H={:.3}, relative={:.3}, bins={}",
        res2.shannon, res2.relative, res2.bin_count
    );
    println!();
}
