use crate::validator::Validator;
use std::collections::HashMap;

pub struct StakeManager {
    validators: HashMap<Vec<u8>, Validator>,
}

impl StakeManager {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
        }
    }

    pub fn add_stake(&mut self, public_key: Vec<u8>, amount: u64) {
        let validator = self
            .validators
            .entry(public_key.clone())
            .or_insert_with(|| Validator::new(public_key, 0));

        if !validator.is_slashed {
            validator.stake += amount;
        }
    }

    /// Calculates voting power using a square root function to dampen whale influence.
    /// Returns floor(sqrt(stake)).
    pub fn get_voting_power(&self, public_key: &[u8]) -> u64 {
        if let Some(validator) = self.validators.get(public_key) {
            if validator.is_slashed {
                return 0;
            }
            // Anti-centralization curve: power = sqrt(stake)
            (validator.stake as f64).sqrt() as u64
        } else {
            0
        }
    }

    pub fn slash(&mut self, public_key: &[u8]) {
        if let Some(validator) = self.validators.get_mut(public_key) {
            validator.is_slashed = true;
            validator.stake = 0; // Full slash for now
        }
    }
}

impl Default for StakeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voting_power_curve() {
        let mut manager = StakeManager::new();
        let pk = vec![1, 2, 3];

        manager.add_stake(pk.clone(), 100);
        assert_eq!(manager.get_voting_power(&pk), 10); // sqrt(100) = 10

        manager.add_stake(pk.clone(), 300); // Total 400
        assert_eq!(manager.get_voting_power(&pk), 20); // sqrt(400) = 20
    }

    #[test]
    fn test_slashing_removes_power() {
        let mut manager = StakeManager::new();
        let pk = vec![1, 2, 3];

        manager.add_stake(pk.clone(), 100);
        assert_eq!(manager.get_voting_power(&pk), 10);

        manager.slash(&pk);
        assert_eq!(manager.get_voting_power(&pk), 0);
        assert_eq!(manager.validators.get(&pk).unwrap().is_slashed, true);
    }
}
