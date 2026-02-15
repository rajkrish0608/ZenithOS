#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

/// Federated Learning Core for Zenith OS.
pub struct FedAvg;

impl FedAvg {
    /// Simulates gradient aggregation from multiple nodes.
    pub fn aggregate(gradients: Vec<Vec<f32>>) -> Vec<f32> {
        if gradients.is_empty() { return Vec::new(); }
        let len = gradients[0].len();
        let mut sum = Vec::with_capacity(len);
        sum.resize(len, 0.0);

        for grad in gradients.iter() {
            for (i, val) in grad.iter().enumerate() {
                sum[i] += val;
            }
        }

        let n = gradients.len() as f32;
        sum.iter().map(|&x| x / n).collect()
    }
}

pub struct DifferentialPrivacy;

impl DifferentialPrivacy {
    /// Injects Laplace noise to satisfy epsilon-differential privacy.
    pub fn inject_noise(weights: &mut [f32], epsilon: f32) {
        // Mock noise injection
        // In reality: adds random values from a Laplace distribution
        let noise_intensity = 1.0 / epsilon;
        for w in weights.iter_mut() {
            *w += 0.001 * noise_intensity; // Simplified mock noise
        }
    }
}
