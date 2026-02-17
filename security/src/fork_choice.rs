use ledger::block::Block;

pub trait ForkChoice {
    fn select_best_chain<'a>(&self, chains: &'a [Vec<Block>]) -> Option<&'a Vec<Block>>;
}

pub struct HeaviestChainRule;

impl HeaviestChainRule {
    pub fn new() -> Self {
        Self
    }
}

// Simplified illustration: In a real system, we'd traverse the block tree.
// Here we compare linear chains by summing the "weight" of blocks.
// For now, let's assume block weight = 1 (Longest Chain) OR we'd look at signatures.
// Improved: Let's pretend each block has a `weight` field or we can calculate it from the validator set.
// For Phase 6 prototype, we'll verify the logic of "selecting the one with higher cumulative score".

impl ForkChoice for HeaviestChainRule {
    fn select_best_chain<'a>(&self, chains: &'a [Vec<Block>]) -> Option<&'a Vec<Block>> {
        if chains.is_empty() {
            return None;
        }

        let mut best_chain: Option<&Vec<Block>> = None;
        let mut max_weight = 0;

        for chain in chains {
            // Calculate chain weight.
            // In a real PoS, this would be sum(validator_stake * votes).
            // For now, we'll use length as a proxy for weight if we don't have explicit votes attached in this struct yet,
            // or we'll define a dummy weight function.

            // Let's make it slightly smarter: cumulative slot difference?
            // Actually, let's stick to Length for now as a baseline, but the Architecture is ready for Weight.
            // To make it "Heaviest", let's assume we have a function `calculate_chain_weight`.

            let weight = self.calculate_chain_weight(chain);

            if weight > max_weight {
                max_weight = weight;
                best_chain = Some(chain);
            }
        }

        best_chain
    }
}

impl HeaviestChainRule {
    fn calculate_chain_weight(&self, chain: &[Block]) -> u64 {
        // Placeholder: Weight = Length * 10
        // In future: Weight = Sum(Block.VoteWeight)
        (chain.len() as u64) * 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ledger::block::{Block, BlockHeader};

    fn mock_block() -> Block {
        Block {
            header: BlockHeader {
                parent_hash: "0".to_string(),
                timestamp: 0,
                slot: 0,
                state_root: "".to_string(),
                transactions_root: "".to_string(),
                validator_public_key: vec![],
                signature: vec![],
            },
            transactions: vec![],
        }
    }

    #[test]
    fn test_heaviest_chain() {
        let rule = HeaviestChainRule::new();

        let chain_short = vec![mock_block()];
        let chain_long = vec![mock_block(), mock_block()];

        // In this mocking, weight = length * 10
        // Short = 10, Long = 20

        let chains = vec![chain_short.clone(), chain_long.clone()];
        let best = rule.select_best_chain(&chains).unwrap();

        assert_eq!(best.len(), 2);
    }
}
