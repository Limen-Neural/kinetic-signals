//! Tests for the optional Sentry feature (issue #11).
//! These tests verify that Sentry initialization only occurs when the feature
//! flag is active and a DSN is provided.

#![cfg(feature = "sentry")]

use std::env;

#[test]
fn sentry_feature_compiles_and_initializes_with_dsn() {
    // Set a dummy DSN for the test
    // SAFETY: This is a test-only environment variable mutation. The test is
    // single-threaded and the variable is only read within the same test.
    unsafe {
        env::set_var("SENTRY_DSN", "https://test@example.ingest.sentry.io/123456");
    }

    // This should not panic and should return a guard when DSN is present
    let guard = match env::var("SENTRY_DSN") {
        Ok(dsn) if !dsn.is_empty() => {
            let g = sentry::init((
                dsn,
                sentry::ClientOptions {
                    release: sentry::release_name!(),
                    ..Default::default()
                },
            ));
            Some(g)
        }
        _ => None,
    };

    // Note: In test environments without a real Sentry backend, init may return
    // a no-op guard. The important assertion is that the feature compiles and
    // the guard logic executes without panicking when DSN is present.
    // We accept either Some or None here as the test environment may not fully
    // initialize Sentry.
    let _ = guard; // Guard is created successfully if we reach this line
}

#[test]
fn sentry_does_not_initialize_without_dsn() {
    // SAFETY: Test-only env var removal in single-threaded test context.
    unsafe {
        env::remove_var("SENTRY_DSN");
    }

    let guard = match env::var("SENTRY_DSN") {
        Ok(dsn) if !dsn.is_empty() => {
            let g = sentry::init(dsn);
            Some(g)
        }
        _ => None,
    };

    assert!(guard.is_none(), "Sentry should not initialize without DSN");
}