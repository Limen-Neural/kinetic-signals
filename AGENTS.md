# kinetic-signals

A Rust library crate (zero required dependencies by default) for streaming signal feature extraction (Hurst exponent, Hawkes process, GBM surprise, volatility, entropy, indicators, stats). Library code lives in `src/`; a runnable demo lives in `examples/demo.rs`.

## Cursor Cloud specific instructions

- This crate uses `edition = "2024"` (see `Cargo.toml`), which requires Rust **>= 1.85**. The current default toolchain (kept current by the startup update script via `rustup`) satisfies this; if you ever see `feature "edition2024" is required`, the active toolchain is too old — run `rustup default stable && rustup update stable`.
- Standard commands (no special setup needed):
  - Build: `cargo build`
  - Test: `cargo test` (runs 16 unit tests plus doctests)
  - Lint: `cargo clippy`
  - Format check: `cargo fmt --check` (note: the committed source currently has minor formatting that does not match `rustfmt`, so this check reports a diff; this is pre-existing and not caused by setup)
  - Run the app/demo: `cargo run --example demo`
- The crate has zero required dependencies by default. The optional `sentry` feature pulls in the `sentry` crate, which requires network access to download.

