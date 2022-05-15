//! Contains [Response] and implementations, see item-level docs for more info

use super::{Action, Message, MessageBytes, ToAction};
use crate::{Error, Result};

/// Response optionally sent back from a peer after a request was received
pub enum Response {
    /// See [Action::PingPong]
    PingPong,
}

impl ToAction for Response {
    fn action(&self) -> Action {
        match self {
            Self::PingPong => Action::PingPong,
        }
    }
}

impl Message for Response {
    fn from_msg(msg_bytes: MessageBytes) -> Result<Self> {
        // Verify length
        if msg_bytes.len() < 1 {
            return Err(Error::Length);
        }

        // Decode
        match Action::from_byte(msg_bytes[0])? {
            Action::PingPong => Ok(Self::PingPong),
        }
    }

    fn to_msg(&self) -> Result<MessageBytes> {
        Ok(match self {
            Self::PingPong => vec![self.action_byte()],
        })
    }
}
