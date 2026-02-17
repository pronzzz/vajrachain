use crate::message::BridgeMessage;
use std::collections::HashSet;

pub struct BridgeState {
    pub chain_id: u64,
    pub processed_nonces: HashSet<u64>, // Anti-replay
    pub outbox: Vec<BridgeMessage>,
}

impl BridgeState {
    pub fn new(chain_id: u64) -> Self {
        Self {
            chain_id,
            processed_nonces: HashSet::new(),
            outbox: Vec::new(),
        }
    }

    /// Helper to Simulate Deposit: User sends tx to bridge contract
    pub fn deposit(
        &mut self,
        dest_chain: u64,
        sender: Vec<u8>,
        recipient: Vec<u8>,
        payload: Vec<u8>,
        nonce: u64,
    ) {
        let msg = BridgeMessage::new(self.chain_id, dest_chain, sender, recipient, payload, nonce);
        // In real system, we emit an Event here.
        // For prototype, we store in outbox for relayer to pick up.
        self.outbox.push(msg);
    }

    /// Process Incoming Message (Withdraw on this chain)
    pub fn process_incoming(&mut self, msg: BridgeMessage) -> Result<(), String> {
        if msg.dest_chain_id != self.chain_id {
            return Err("Wrong destination chain".to_string());
        }

        if self.processed_nonces.contains(&msg.nonce) {
            return Err("Message already processed (Replay detected)".to_string());
        }

        // Verify proof would happen here (e.g. check signature of validator set of source chain)
        // For prototype, we assume the message is valid if it reached here via trusted relayer simulation.

        self.processed_nonces.insert(msg.nonce);

        // Execute payload (mint tokens, call contract etc)
        // println!("Executed message from chain {}: {:?}", msg.source_chain_id, msg.payload);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit() {
        let mut bridge = BridgeState::new(1);
        bridge.deposit(2, vec![0x1], vec![0x2], vec![0x3], 1);
        assert_eq!(bridge.outbox.len(), 1);
        assert_eq!(bridge.outbox[0].nonce, 1);
    }

    #[test]
    fn test_replay_protection() {
        let mut bridge = BridgeState::new(1);
        let msg = BridgeMessage::new(2, 1, vec![], vec![], vec![], 5);

        assert!(bridge.process_incoming(msg.clone()).is_ok());
        assert!(bridge.process_incoming(msg).is_err()); // Replay
    }
}
