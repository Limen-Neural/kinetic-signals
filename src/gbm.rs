//! Deprecated financial-domain aliases for the generic [`crate::surprise`] module.
//!
//! The names in this module assumed a Geometric Brownian Motion (financial)
//! framing. They are retained as thin, backward-compatible wrappers and will be
//! removed in a future release. Prefer the domain-agnostic names in
//! [`crate::surprise`].
#![allow(deprecated)]

use crate::real::Real;
use crate::surprise::{self, SurpriseParams, SurpriseResult};

/// Deprecated alias for [`crate::surprise::SurpriseResult`].
#[deprecated(
    since = "0.3.0",
    note = "use `kinetic_signals::surprise::SurpriseResult`"
)]
pub type GBMResult<T = f64> = SurpriseResult<T>;

/// Deprecated alias for [`crate::surprise::SurpriseParams`].
#[deprecated(
    since = "0.3.0",
    note = "use `kinetic_signals::surprise::SurpriseParams`"
)]
pub type GBMParams<T = f64> = SurpriseParams<T>;

/// Deprecated alias for [`crate::surprise::compute_surprise`].
#[deprecated(
    since = "0.3.0",
    note = "use `kinetic_signals::surprise::compute_surprise`"
)]
pub fn compute_gbm_surprise<T>(
    current_value: T,
    previous_value: T,
    params: &SurpriseParams<T>,
) -> SurpriseResult<T>
where
    T: Real,
{
    surprise::compute_surprise(current_value, previous_value, params)
}

/// Deprecated alias for [`crate::surprise::compute_surprise_sequence`].
#[deprecated(
    since = "0.3.0",
    note = "use `kinetic_signals::surprise::compute_surprise_sequence`"
)]
pub fn compute_gbm_surprise_sequence<T>(
    values: &[T],
    params: &SurpriseParams<T>,
) -> Vec<SurpriseResult<T>>
where
    T: Real,
{
    surprise::compute_surprise_sequence(values, params)
}

/// Deprecated alias for [`crate::surprise::detect_anomaly`].
#[deprecated(
    since = "0.3.0",
    note = "use `kinetic_signals::surprise::detect_anomaly`"
)]
pub fn detect_anomaly<T>(result: &SurpriseResult<T>, params: &SurpriseParams<T>) -> bool
where
    T: Real,
{
    surprise::detect_anomaly(result, params)
}
