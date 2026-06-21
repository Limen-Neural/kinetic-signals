//! Backward-compatibility checks for the deprecated financial aliases (issue #4).
//!
//! The GBM-named items are deprecated but must keep working and must produce
//! results identical to the domain-agnostic API they forward to.
#![allow(deprecated)]

use kinetic_signals::{
    GBMParams, compute_gbm_surprise, compute_surprise, surprise::SurpriseParams,
};

#[test]
fn gbm_aliases_match_generic_api() {
    let generic = SurpriseParams::<f64>::default();
    let legacy = GBMParams::<f64>::default();

    let a = compute_surprise(150.0, 100.0, &generic);
    let b = compute_gbm_surprise(150.0, 100.0, &legacy);

    assert_eq!(a.surprise, b.surprise);
    assert_eq!(a.z_score, b.z_score);
    assert_eq!(a.log_return, b.log_return);
    assert_eq!(a.expected_return, b.expected_return);
}
