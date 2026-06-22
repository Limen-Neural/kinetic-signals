// SPDX-License-Identifier: MIT OR Apache-2.0

#[derive(Debug, Clone)]
pub struct HawkesResult {
    pub intensity: f64,
    pub event_count: usize,
    pub avg_excitation: f64,
}

#[derive(Debug, Clone)]
pub struct HawkesParams {
    pub mu: f64,
    pub alpha: f64,
    pub beta: f64,
    pub dt: f64,
}

impl Default for HawkesParams {
    fn default() -> Self {
        HawkesParams {
            mu: 0.1,
            alpha: 0.5,
            beta: 1.0,
            dt: 0.001,
        }
    }
}

pub fn compute_hawkes(event_times: &[f64], params: &HawkesParams) -> HawkesResult {
    if event_times.is_empty() {
        return HawkesResult {
            intensity: params.mu,
            event_count: 0,
            avg_excitation: 0.0,
        };
    }

    let mut excitations: Vec<f64> = Vec::new();

    let &last_time = event_times.last().unwrap();

    for &t in event_times {
        let excitation = params.alpha * (-params.beta * (last_time - t)).exp();
        excitations.push(excitation);
    }

    let intensity = params.mu + excitations.iter().sum::<f64>();

    let avg_excitation = if excitations.is_empty() {
        0.0
    } else {
        excitations.iter().sum::<f64>() / excitations.len() as f64
    };

    HawkesResult {
        intensity,
        event_count: event_times.len(),
        avg_excitation,
    }
}

pub fn compute_hawkes_streaming(
    _prev_intensity: f64,
    new_event_time: f64,
    last_event_time: f64,
    params: &HawkesParams,
    decay_sum: f64,
) -> (f64, f64) {
    let dt = new_event_time - last_event_time;

    let decayed_sum = decay_sum * (-params.beta * dt).exp();

    let new_intensity = params.mu + params.alpha * decayed_sum;

    let new_decay_sum = decayed_sum + 1.0;

    (new_intensity, new_decay_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hawkes_basic() {
        let events = vec![0.0, 0.1, 0.2, 0.3, 0.4];
        let params = HawkesParams::default();
        let result = compute_hawkes(&events, &params);
        assert!(result.intensity > params.mu);
        assert_eq!(result.event_count, 5);
    }

    #[test]
    fn test_hawkes_empty() {
        let events: Vec<f64> = vec![];
        let params = HawkesParams::default();
        let result = compute_hawkes(&events, &params);
        assert_eq!(result.intensity, params.mu);
        assert_eq!(result.event_count, 0);
    }
}
