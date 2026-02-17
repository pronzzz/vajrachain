use crate::registry::SystemContract;
use storage::state_compliance::AccountState;

pub trait NativeExecution {
    fn execute(
        &self,
        contract: SystemContract,
        input: &[u8],
        state: &mut AccountState,
    ) -> Result<Vec<u8>, String>;
}

pub struct NativeExecutor;

impl NativeExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl NativeExecution for NativeExecutor {
    fn execute(
        &self,
        contract: SystemContract,
        input: &[u8],
        state: &mut AccountState,
    ) -> Result<Vec<u8>, String> {
        match contract {
            SystemContract::Staking => {
                // Mock Staking Logic:
                // Input: [Action(1 byte), Amount(8 bytes)]
                // If Action == 0x01 (Stake), deduct balance (simulated)
                // This is a simplified example.
                if input.len() >= 1 && input[0] == 0x01 {
                    // Logic to lock funds/update nonce would go here
                    // verifying state.balance sufficient etc.
                    state.nonce += 1; // dummy side effect
                    Ok(b"Staked".to_vec())
                } else {
                    Err("Invalid staking input".to_string())
                }
            }
            _ => Ok(b"NoOp".to_vec()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staking_execution() {
        let mut state = AccountState::new(1000, 0, 0);
        let executor = NativeExecutor::new();

        // Action 0x01 = Stake
        let input = vec![0x01];
        let res = executor.execute(SystemContract::Staking, &input, &mut state);

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), b"Staked".to_vec());
        assert_eq!(state.nonce, 1);
    }
}
