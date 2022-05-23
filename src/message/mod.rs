//! Messaging constructs which enable communication between peers
//!
//! - Cryptography
//!   - [Key]
//! - Packets
//!   - [PacketBytes]
//!   - [MessageBytes]
//! - Actions
//!   - [Action]
//!   - [ToAction]
//! - Messaging
//!   - [Message]
//!   - [Request]
//!   - [Response]

mod action;
mod request;
mod response;

pub use action::{Action, ToAction};
pub use request::Request;
pub use response::Response;

use crate::{Error, Result};
use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::{Aes256GcmSiv, Nonce};
use rand::prelude::*;

/// Length of nonces used for [KeyNonce] security
const NONCE_LEN: usize = 12;

/// Nonce used for [Key] encryption/decryption security
type KeyNonce = [u8; NONCE_LEN];

/// Encryption key for AES256-based packets
#[derive(Clone)]
pub struct Key(aes_gcm_siv::aead::Key<Aes256GcmSiv>);

impl Key {
    /// Encrypts [MessageBytes] into [PacketBytes]
    pub fn encrypt(&self, msg_bytes: MessageBytes) -> Result<PacketBytes> {
        let cipher = self.cipher();
        let nonce: KeyNonce = thread_rng().gen();
        let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce[..]), &msg_bytes[..])?;

        Ok([nonce.to_vec(), ciphertext].concat())
    }

    /// Decrypts a [PacketBytes] into a [MessageBytes]
    pub fn decrypt(&self, packet_bytes: PacketBytes) -> Result<MessageBytes> {
        // Verify
        if packet_bytes.len() < NONCE_LEN {
            return Err(Error::Length);
        }

        // Decrypt
        let cipher = self.cipher();
        let nonce: KeyNonce = packet_bytes[..NONCE_LEN].try_into().unwrap();
        let encrypted = &packet_bytes[NONCE_LEN..];
        let decrypted = cipher.decrypt(Nonce::from_slice(&nonce[..]), encrypted)?;

        Ok(decrypted)
    }

    /// Constructs the cipher used for encryption/decryption
    fn cipher(&self) -> Aes256GcmSiv {
        Aes256GcmSiv::new(&self.0)
    }
}

/// Alias to an encrypted version of [MessageBytes]
pub type PacketBytes = Vec<u8>;

/// Alias to a decrypted version of [PacketBytes]
pub type MessageBytes = Vec<u8>;

/// Two-way messaging constructs, allowing encoding/encryption and decoding/decryption
pub trait Message: Sized {
    /// Decodes message into self; used in [Self::from_packet]
    fn from_msg(msg_bytes: MessageBytes) -> Result<Self>;

    /// Encodes self into a message ready to be encrypted and sent; used in [Self::to_packet]
    fn to_msg(&self) -> Result<MessageBytes>;

    /// Fully decrypts and decodes a packet from start to finish, resulting in self
    fn from_packet(key: &Key, packet_bytes: PacketBytes) -> Result<Self> {
        Self::from_msg(key.decrypt(packet_bytes)?)
    }

    /// Fully encodes and encrypts self into a packet ready to be sent
    fn to_packet(&self, key: &Key) -> Result<PacketBytes> {
        key.encrypt(self.to_msg()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aes_gcm_siv::Key;

    #[test]
    fn cyclic() {
        // Setup
        let data = vec![
            3, 45, 4, 34, 5, 4, 35, 3, 45, 43, 54, 35, 34, 4, 4, 4, 4, 44, 3, 3, 3, 2, 21, 12, 43,
            5, 54, 65, 67, 56, 7, 65, 74, 44,
        ];
        let key = Key::from_slice(b"an example very very secret key.");
        let key = Key(*key);

        // Encrypt & decrypt
        let encrypted = key.encrypt(data.clone()).unwrap();
        let decrypted = key.decrypt(encrypted).unwrap();
        assert_eq!(decrypted, data);
    }
}
