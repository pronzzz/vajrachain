use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Key generation failed")]
    KeyGenerationFailed,
    #[error("Serialization error")]
    SerializationError,
}

pub trait Hashable {
    fn hash(&self) -> String;
}

pub struct Blake3;

impl Blake3 {
    pub fn hash(data: &[u8]) -> String {
        let hash = blake3::hash(data);
        hash.to_hex().to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyPair {
    pub public_key: Vec<u8>,
    // In a real scenario, keep private key secure/secret
    pub private_key: Vec<u8>,
}

impl KeyPair {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        Self {
            public_key: verifying_key.to_bytes().to_vec(),
            private_key: signing_key.to_bytes().to_vec(),
        }
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let signing_key = SigningKey::from_bytes(self.private_key.as_slice().try_into().unwrap());
        let signature = signing_key.sign(message);
        Ok(signature.to_bytes().to_vec())
    }

    pub fn verify(
        public_key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> Result<bool, CryptoError> {
        let verifying_key = VerifyingKey::from_bytes(
            public_key
                .try_into()
                .map_err(|_| CryptoError::SerializationError)?,
        )
        .map_err(|_| CryptoError::SerializationError)?;

        let signature = Signature::from_bytes(
            signature
                .try_into()
                .map_err(|_| CryptoError::SerializationError)?,
        );

        verifying_key
            .verify(message, &signature)
            .map(|_| true)
            .map_err(|_| CryptoError::InvalidSignature)
    }
}
