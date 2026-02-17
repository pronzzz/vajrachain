use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BridgeMessage {
    pub source_chain_id: u64,
    pub dest_chain_id: u64,
    pub sender: Vec<u8>,
    pub recipient: Vec<u8>,
    pub payload: Vec<u8>,
    pub nonce: u64,
}

impl BridgeMessage {
    pub fn new(
        source: u64,
        dest: u64,
        sender: Vec<u8>,
        recipient: Vec<u8>,
        payload: Vec<u8>,
        nonce: u64,
    ) -> Self {
        Self {
            source_chain_id: source,
            dest_chain_id: dest,
            sender,
            recipient,
            payload,
            nonce,
        }
    }

    /// Serializes the message for signing/hashing
    pub fn serialize(&self) -> Vec<u8> {
        // Simple serialization: concatenation or usage of serde_json/bincode
        // For prototype, we use a simple debug format or json
        serde_json::to_vec(self).unwrap_or_default()
    }
}
