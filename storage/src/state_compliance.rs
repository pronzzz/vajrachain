use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountState {
    pub balance: u64,
    pub nonce: u64,
    pub last_rent_paid_epoch: u64,
    pub code_hash: Vec<u8>,    // For Smart Contracts
    pub storage_root: Vec<u8>, // For Contract Storage
}

impl AccountState {
    pub fn new(balance: u64, nonce: u64, current_epoch: u64) -> Self {
        Self {
            balance,
            nonce,
            last_rent_paid_epoch: current_epoch,
            code_hash: Vec::new(),
            storage_root: Vec::new(),
        }
    }

    /// Applies rent deduction based on epochs passed since last payment.
    /// Returns true if account still has funds/exists, false if it should be pruned (balance <= 0).
    pub fn apply_rent(&mut self, current_epoch: u64, rent_per_epoch: u64) -> bool {
        if current_epoch <= self.last_rent_paid_epoch {
            return true;
        }

        let epochs_passed = current_epoch - self.last_rent_paid_epoch;
        let total_rent = epochs_passed * rent_per_epoch;

        if total_rent >= self.balance {
            self.balance = 0;
            false // Account effectively dead/pruned
        } else {
            self.balance -= total_rent;
            self.last_rent_paid_epoch = current_epoch;
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rent_deduction() {
        let mut account = AccountState::new(1000, 0, 10);
        let active = account.apply_rent(15, 10);
        assert!(active);
        assert_eq!(account.balance, 950);
        assert_eq!(account.last_rent_paid_epoch, 15);
    }

    #[test]
    fn test_rent_pruning() {
        let mut account = AccountState::new(100, 0, 10);
        let active = account.apply_rent(30, 10);
        assert!(!active);
        assert_eq!(account.balance, 0);
    }
}
