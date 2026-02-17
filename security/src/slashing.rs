#[derive(Debug, Clone, PartialEq)]
pub enum Misbehavior {
    DoubleSign,
    Downtime(u64), // number of blocks missed
}

pub struct SlashingManager {
    // Configuration
    pub double_sign_penalty_percent: u8,
    pub downtime_penalty_per_block: u64, // simplified flat amount per block
}

impl SlashingManager {
    pub fn new(double_sign_penalty_percent: u8, downtime_penalty_per_block: u64) -> Self {
        Self {
            double_sign_penalty_percent,
            downtime_penalty_per_block,
        }
    }

    pub fn calculate_penalty(&self, misbehavior: Misbehavior, stake: u64) -> u64 {
        match misbehavior {
            Misbehavior::DoubleSign => {
                // Slash X% of stake
                let penalty = (stake as u128 * self.double_sign_penalty_percent as u128) / 100;
                penalty as u64
            }
            Misbehavior::Downtime(blocks_missed) => {
                // Slash flat amount * blocks
                blocks_missed * self.downtime_penalty_per_block
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_sign_penalty() {
        let manager = SlashingManager::new(5, 100); // 5% penalty
        let stake = 1000;
        let penalty = manager.calculate_penalty(Misbehavior::DoubleSign, stake);
        assert_eq!(penalty, 50); // 5% of 1000 is 50
    }

    #[test]
    fn test_downtime_penalty() {
        let manager = SlashingManager::new(5, 10); // 10 per block
        let penalty = manager.calculate_penalty(Misbehavior::Downtime(5), 0);
        assert_eq!(penalty, 50); // 5 blocks * 10 = 50
    }
}
