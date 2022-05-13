use std::{fmt, io};

/// Alias for results which may end up as an operation error
pub type Result<T> = std::result::Result<T, Error>;

/// Library error variants for anything which could go wrong during operation
#[derive(Debug)]
pub enum Error {
    Bind(io::Error),
    Send(io::Error),
    Receive(io::Error),
    Length,
    Encryption(aes_gcm_siv::aead::Error),
    Action(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bind(err) => write!(f, "Whilst binding, {}", err),
            Self::Send(err) => write!(f, "Whilst sending, {}", err),
            Self::Receive(err) => write!(f, "Whilst receiving, {}", err),
            Self::Length => write!(f, "Incoming message is too short"),
            Self::Encryption(err) => write!(f, "Couldn't encrypt/decrypt, {}", err),
            Self::Action(unknown) => write!(f, "Unknown message action #{}", unknown),
        }
    }
}

impl From<aes_gcm_siv::aead::Error> for Error {
    fn from(err: aes_gcm_siv::aead::Error) -> Self {
        Self::Encryption(err)
    }
}
