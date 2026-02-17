use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DIDDocument {
    pub id: String,
    pub public_key: Vec<u8>,
    pub controller: Option<String>,
}

impl DIDDocument {
    pub fn new(public_key: Vec<u8>) -> Self {
        // Simple method: did:vajra:<hex(pubkey)>
        let id = format!("did:vajra:{}", hex::encode(&public_key));
        Self {
            id,
            public_key,
            controller: None,
        }
    }

    pub fn resolve(did: &str) -> Option<Self> {
        // Mock resolution: In a real system, this would query the ledger/state.
        // For now, if it matches valid format, we reconstruct it (stateless for this prototype).
        if did.starts_with("did:vajra:") {
            let parts: Vec<&str> = did.split(':').collect();
            if parts.len() == 3 {
                if let Ok(pubkey) = hex::decode(parts[2]) {
                    return Some(Self::new(pubkey));
                }
            }
        }
        None
    }
}

impl fmt::Display for DIDDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_did_creation() {
        let pubkey = vec![0x01, 0x02, 0x03];
        let did = DIDDocument::new(pubkey.clone());
        assert_eq!(did.public_key, pubkey);
        assert!(did.id.starts_with("did:vajra:"));
    }

    #[test]
    fn test_did_resolve() {
        let pubkey = vec![0x01, 0x02, 0x03];
        let did_str = format!("did:vajra:{}", hex::encode(&pubkey));
        let resolved = DIDDocument::resolve(&did_str).unwrap();
        assert_eq!(resolved.public_key, pubkey);
    }
}
