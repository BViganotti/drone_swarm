use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::rngs::OsRng;
use rand::RngCore;

use crate::error::{DroneCommError, Result};

pub fn generate_encryption_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

pub fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

pub fn encrypt_payload(cipher: &Aes256Gcm, nonce: &[u8; 12], payload: &[u8]) -> Result<Vec<u8>> {
    cipher
        .encrypt(Nonce::from_slice(nonce), payload)
        .map_err(|e| DroneCommError::EncryptionError(e.to_string()))
}

pub fn decrypt_payload(cipher: &Aes256Gcm, nonce: &[u8; 12], encrypted: &[u8]) -> Result<Vec<u8>> {
    cipher
        .decrypt(Nonce::from_slice(nonce), encrypted)
        .map_err(|e| DroneCommError::DecryptionError(e.to_string()))
}
