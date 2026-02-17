use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    pub id: String,
    pub issuer: String,  // DID of issuer
    pub subject: String, // DID of subject
    pub claims: String,  // JSON string of claims
    pub proof: Vec<u8>,  // Signature
}

impl VerifiableCredential {
    pub fn new(
        id: String,
        issuer: String,
        subject: String,
        claims: String,
        proof: Vec<u8>,
    ) -> Self {
        Self {
            id,
            issuer,
            subject,
            claims,
            proof,
        }
    }

    pub fn verify(&self, issuer_public_key: &[u8]) -> bool {
        // Mock verification: In real system, verify signature over (id+issuer+subject+claims) using pubkey.
        // For prototype, just check if proof is not empty.
        !self.proof.is_empty() && !issuer_public_key.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_verification() {
        let vc = VerifiableCredential::new(
            "vc:1".to_string(),
            "did:vajra:issuer".to_string(),
            "did:vajra:subject".to_string(),
            "{\"role\": \"admin\"}".to_string(),
            vec![1, 2, 3],
        );
        assert!(vc.verify(&[0x01]));
        assert!(!vc.verify(&[]));
    }
}
