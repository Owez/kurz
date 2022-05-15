use super::{Action, Message, MessageBytes, ToAction};
use crate::{Error, Result};

/// Requests which are sent to other peers on the network, optionally expecting a response
pub enum Request {
    // See [Action::PingPong]
    PingPong,
}

impl ToAction for Request {
    fn action(&self) -> Action {
        match self {
            Self::PingPong => Action::PingPong,
        }
    }
}

impl Message for Request {
    fn to_msg(&self) -> Result<MessageBytes> {
        Ok(match self {
            Self::PingPong => vec![self.action_byte()],
        })
    }

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
}
