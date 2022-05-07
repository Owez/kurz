mod action;

pub use action::{Action, ToAction};

use crate::Result;

/// Encryption key for AES256-based packets
pub struct Key(aes_gcm_siv::Aes256GcmSiv);

impl Key {
    /// Encrypts [MessageBytes] into [PacketBytes]
    pub fn encrypt(&self, msg_bytes: MessageBytes) -> Result<PacketBytes> {
        todo!()
    }

    /// Decrypts a [PacketBytes] into a [MessageBytes]
    pub fn decrypt(&self, packet_bytes: PacketBytes) -> Result<MessageBytes> {
        todo!()
    }
}

/// Alias to an encrypted version of [MessageBytes]
pub type PacketBytes = Vec<u8>;

/// Alias to a decrypted version of [PacketBytes]
pub type MessageBytes = Vec<u8>;

pub trait Message: Sized {
    /// TODO: document
    fn from_msg(msg_bytes: MessageBytes) -> Result<Self>;

    /// TODO: document
    fn to_msg(&self) -> Result<MessageBytes>;

    /// TODO: document
    fn from_packet(key: &Key, packet_bytes: PacketBytes) -> Result<Self> {
        Self::from_msg(key.decrypt(packet_bytes)?)
    }
    /// TODO: document
    fn to_packet(&self, key: &Key) -> Result<PacketBytes> {
        key.encrypt(self.to_msg()?)
    }
}
