use serde::{Deserialize, Serialize};

/// Represents an input in the UTXO model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UtxoInput {
    pub transaction_hash: String,
    pub output_index: u32,
    pub signature: Vec<u8>, // ScriptSig effectively
    pub public_key: Vec<u8>,
}

/// Represents an output in the UTXO model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UtxoOutput {
    pub amount: u64,
    pub recipient_address: String,
}

/// Represents an account-based transaction (like Ethereum).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountTransaction {
    pub nonce: u64,
    pub to: Option<String>, // None for contract creation
    pub amount: u64,
    pub data: Vec<u8>,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub signature: Vec<u8>,
    pub sender_public_key: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UtxoTransaction {
    pub inputs: Vec<UtxoInput>,
    pub outputs: Vec<UtxoOutput>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionData {
    Utxo(UtxoTransaction),
    Account(AccountTransaction),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub data: TransactionData,
}

impl Transaction {
    // Helper functionality for serializing and hashing would go here
}
