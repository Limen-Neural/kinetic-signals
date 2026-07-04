# kinetic-signals — Boundary Matrix

Tracked by: Linear LIM-9 | GitHub #10

## Purpose

Domain-agnostic streaming feature extraction for stochastic signals. Computes real-time statistics, point-process intensity, and anomaly metrics on high-velocity time-series data.

**Not** a spike-train analyzer, financial domain adapter, or SNN runtime.

## Owns

| Area | What belongs here |
|------|-------------------|
| Hurst exponent | Long-memory / persistence detection (R/S method) |
| Hawkes process | Self-exciting point-process intensity estimation |
| Surprise | Normalized log-ratio z-score anomaly detection |
| Volatility | Rolling ring-buffer RMS volatility |
| Shannon entropy | Signal complexity / information density |
| Indicators | EMA, SMA, Z-score tracking |
| Signal stats | High-order moments (skewness, kurtosis) |

## Does NOT own

| Area | Belongs to |
|------|------------|
| Spike-train specific analysis (ISI, PSTH, raster) | SpikeStream.jl |
| SNN runtime / neuron models / synaptic dynamics | neuromod |
| Financial domain adapters (portfolio, PnL, Greeks) | DendriteTrader.jl / metabolic-ledger |
| Hardware signal acquisition | silicon-bridge |
| Supervisory orchestration | brainstem-daemon |
| Domain-specific anomaly policies | Application-layer crates |

## Dependencies

### Allowed

| Dependency | Reason |
|------------|--------|
| `sentry` (optional) | Error monitoring, feature-gated |
| `temp-env` (dev) | Safe env var testing |
| `serial_test` (dev) | Test serialization |

### Forbidden (by design)

| Dependency | Why forbidden |
|------------|---------------|
| `neuromod` | Would create circular dependency; kinetic-signals is leaf crate |
| `SpikeStream` bindings | Julia crate, different language boundary |
| Financial domain crates | Out of scope for generic signal processing |
| Heavy ML frameworks | Keeps crate zero-required-dependency |

## Cross-repo handoff points

| Consumer | Integration |
|----------|-------------|
| SpikeStream.jl | Uses `compute_hurst`, `compute_hawkes`, `surprise` via FFI or Julia bindings. Shares `tests/fixtures/shared_vectors.json` for cross-language parity. |
| neuromod | May re-export `compute_hurst` / `compute_hawkes` for SNN signal analysis. No direct dependency — re-export only. |
| DendriteTrader.jl | Can use surprise / volatility for financial signal detection. Domain adapter layer lives in DendriteTrader, not here. |

## Thread-safety

All public types are `Send + Sync` (no interior mutability). `VolEstimator` mutates through `&mut self`; if you need to update a single instance from multiple threads, use normal Rust synchronization (e.g., wrap it in a `Mutex`) or keep per-thread estimators. Pure compute functions are stateless and safe for parallel use.

## Domain leaks / migration risks

- **Transitional financial proxies (migration risk):** GBM-named aliases (`GBMParams`, `GBMResult`, `compute_gbm_surprise*`) are still publicly re-exported in v0.3.x as deprecated compatibility shims. They should remain thin name-only forwards to the generic API and are planned for removal in a future release once downstream consumers migrate.
- **SpikeStream.jl migration open question:** SpikeStream.jl issues reference transitional financial proxy naming during the migration away from GBM terminology; ensure this crate does not accrete new financial semantics while supporting that transition.
- Future domain-specific features (e.g., financial Greeks, spike ISI) should be added in consumer crates, not here.
- If a feature is requested that requires domain knowledge, redirect to the appropriate consumer crate.

## Sequencing

1. PR #12 (dual-license + sentry) — merged
2. PR #16 (README dev section) — open
3. PR #17 (remove deprecated aliases) — open
4. This document (PR #18) — open
5. crates.io publishing — deferred until repo quality is satisfactory
