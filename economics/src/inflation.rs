pub struct InflationManager {
    pub initial_reward: u64,
    pub decay_rate: f64, // e.g. 0.95 for 5% decay per epoch/year
}

impl InflationManager {
    pub fn new(initial_reward: u64, decay_rate: f64) -> Self {
        Self {
            initial_reward,
            decay_rate,
        }
    }

    pub fn calculate_reward(&self, epoch: u64) -> u64 {
        // Simple geometric decay: Reward = Initial * (Decay ^ Epoch)
        // Note: Using f64 for simplicity in prototype, real blockchain would use fixed-point arithmetic.
        let reward = (self.initial_reward as f64) * self.decay_rate.powf(epoch as f64);
        reward as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inflation_decay() {
        let manager = InflationManager::new(1000, 0.5);

        // Epoch 0: 1000 * 0.5^0 = 1000
        assert_eq!(manager.calculate_reward(0), 1000);

        // Epoch 1: 1000 * 0.5^1 = 500
        assert_eq!(manager.calculate_reward(1), 500);

        // Epoch 2: 1000 * 0.5^2 = 250
        assert_eq!(manager.calculate_reward(2), 250);
    }
}
