// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for the optional Sentry feature (issue #11).
//! These tests verify that Sentry initialization only occurs when the feature
//! flag is active and a DSN is provided.

#![cfg(feature = "sentry")]

use serial_test::serial;

#[test]
#[serial]
fn sentry_feature_compiles_and_initializes_with_dsn() {
    temp_env::with_vars(
        [(
            "SENTRY_DSN",
            Some("https://test@example.ingest.sentry.io/123456"),
        )],
        || {
            let guard = kinetic_signals::init_sentry();
            assert!(guard.is_some(), "Sentry should initialize when DSN is set");
        },
    );
}

#[test]
#[serial]
fn sentry_does_not_initialize_without_dsn() {
    temp_env::with_vars([("SENTRY_DSN", None::<&str>)], || {
        let guard = kinetic_signals::init_sentry();
        assert!(guard.is_none(), "Sentry should not initialize without DSN");
    });
}
