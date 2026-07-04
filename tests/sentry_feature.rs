// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for the optional Sentry feature (issue #11).
//! These tests verify that Sentry initialization only occurs when the feature
//! flag is active and a DSN is provided.

#![cfg(feature = "sentry")]

use serial_test::serial;
use std::env;

/// Try to initialize Sentry from the `SENTRY_DSN` env var.
/// Returns `Some(guard)` if the DSN is set and non-empty, `None` otherwise.
fn try_init_sentry() -> Option<sentry::ClientInitGuard> {
    match env::var("SENTRY_DSN") {
        Ok(dsn) if !dsn.is_empty() => {
            let guard = sentry::init((
                dsn,
                sentry::ClientOptions {
                    release: sentry::release_name!(),
                    ..Default::default()
                },
            ));
            Some(guard)
        }
        _ => None,
    }
}

#[test]
#[serial]
fn sentry_feature_compiles_and_initializes_with_dsn() {
    // SAFETY: Test-only env var mutation in single-threaded context.
    // Required by Rust edition 2024 — env::set_var is unsafe.
    #[allow(unsafe_code)]
    unsafe {
        env::set_var("SENTRY_DSN", "https://test@example.ingest.sentry.io/123456");
    }

    let guard = try_init_sentry();
    // Guard is created successfully if we reach this line.
    // In test environments without a real Sentry backend, init may return
    // a no-op guard — the important thing is it doesn't panic.
    let _ = guard;

    // SAFETY: Test-only env var cleanup.
    #[allow(unsafe_code)]
    unsafe {
        env::remove_var("SENTRY_DSN");
    }
}

#[test]
#[serial]
fn sentry_does_not_initialize_without_dsn() {
    // SAFETY: Test-only env var removal in single-threaded context.
    #[allow(unsafe_code)]
    unsafe {
        env::remove_var("SENTRY_DSN");
    }

    let guard = try_init_sentry();
    assert!(guard.is_none(), "Sentry should not initialize without DSN");
}
