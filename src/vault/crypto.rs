use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::Argon2;
use rand::{rngs::OsRng, RngCore};
use std::fmt;

pub const SALT_LEN: usize = 16;
pub const NONCE_LEN: usize = 12;

#[derive(Debug)]
pub enum CryptoError {
    KeyDerivationFailed,
    EncryptionFailed,
    DecryptionFailed,
    InvalidDataLength,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::KeyDerivationFailed => write!(f, "Failed to derive vault master key"),
            CryptoError::EncryptionFailed => write!(f, "Failed to encrypt data"),
            CryptoError::DecryptionFailed => write!(f, "Failed to decrypt data or authentication tag mismatch"),
            CryptoError::InvalidDataLength => write!(f, "Encrypted payload length is invalid"),
        }
    }
}

impl std::error::Error for CryptoError {}

pub struct VaultCrypto {
    cipher: Aes256Gcm,
}

impl VaultCrypto {
    pub fn new_from_passphrase(passphrase: &str, salt: Option<&[u8]>) -> Result<(Self, Vec<u8>), CryptoError> {
        let actual_salt = match salt {
            Some(s) => s.to_vec(),
            None => {
                let mut random_salt = vec![0u8; SALT_LEN];
                OsRng.fill_bytes(&mut random_salt);
                random_salt
            }
        };

        let mut key = [0u8; 32];
        let argon2 = Argon2::default();
        
        argon2
            .hash_password_into(passphrase.as_bytes(), &actual_salt, &mut key)
            .map_err(|_| CryptoError::KeyDerivationFailed)?;

        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|_| CryptoError::KeyDerivationFailed)?;

        Ok((Self { cipher }, actual_salt))
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self.cipher
            .encrypt(nonce, plaintext)
            .map_err(|_| CryptoError::EncryptionFailed)?;

        let mut result = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    pub fn decrypt(&self, payload: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if payload.len() < NONCE_LEN {
            return Err(CryptoError::InvalidDataLength);
        }

        let (nonce_bytes, ciphertext) = payload.split_at(NONCE_LEN);
        let nonce = Nonce::from_slice(nonce_bytes);

        self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| CryptoError::DecryptionFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_encryption_decryption() {
        let passphrase = "nexora_ultra_secure_local_key_2026";
        let (crypto, salt) = VaultCrypto::new_from_passphrase(passphrase, None).unwrap();

        let secret_message = b"Secret user browsing history: https://nexora.local";
        let encrypted = crypto.encrypt(secret_message).unwrap();

        assert_ne!(encrypted, secret_message);

        let (crypto_restore, _) = VaultCrypto::new_from_passphrase(passphrase, Some(&salt)).unwrap();
        let decrypted = crypto_restore.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, secret_message);
    }

    #[test]
    fn test_invalid_key_fails_decryption() {
        let passphrase = "correct_passphrase";
        let (crypto, salt) = VaultCrypto::new_from_passphrase(passphrase, None).unwrap();
        let encrypted = crypto.encrypt(b"Confidential data").unwrap();

        let (wrong_crypto, _) = VaultCrypto::new_from_passphrase("wrong_passphrase", Some(&salt)).unwrap();
        assert!(wrong_crypto.decrypt(&encrypted).is_err());
    }
}
