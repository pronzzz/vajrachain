use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemContract {
    Staking,
    Governance,
    IdentityRegistry,
}

impl SystemContract {
    pub fn from_address(addr: &[u8]) -> Option<Self> {
        // Simple mapping:
        // 0x01 -> Staking
        // 0x02 -> Governance
        // 0x03 -> Identity
        if addr.len() == 1 {
            match addr[0] {
                0x01 => Some(Self::Staking),
                0x02 => Some(Self::Governance),
                0x03 => Some(Self::IdentityRegistry),
                _ => None,
            }
        } else {
            None
        }
    }
}
