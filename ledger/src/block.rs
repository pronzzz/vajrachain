use crate::merkle::compute_merkle_root;
use crate::transaction::Transaction;
use crypto::Blake3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockHeader {
    pub parent_hash: String,
    pub timestamp: u64,
    pub slot: u64,
    pub state_root: String,
    pub transactions_root: String,
    pub validator_public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        parent_hash: String,
        timestamp: u64,
        slot: u64,
        state_root: String,
        transactions: Vec<Transaction>,
        validator_public_key: Vec<u8>,
    ) -> Self {
        // Compute transactions root
        let tx_hashes: Vec<String> = transactions
            .iter()
            .map(|tx| {
                let serialized = bincode::serialize(tx).unwrap(); // Should handle error safely in prod
                Blake3::hash(&serialized)
            })
            .collect();

        let transactions_root = compute_merkle_root(&tx_hashes);

        let header = BlockHeader {
            parent_hash,
            timestamp,
            slot,
            state_root,
            transactions_root,
            validator_public_key,
            signature: Vec::new(), // To be signed
        };

        Block {
            header,
            transactions,
        }
    }

    pub fn hash(&self) -> String {
        let serialized = bincode::serialize(&self.header).unwrap();
        Blake3::hash(&serialized)
    }
}
