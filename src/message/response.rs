//! Contains [Response] and implementations, see item-level docs for more info

use super::{Action, Message, MessageBytes, ToAction};
use crate::{Error, Result};

/// Response optionally sent back from a peer after a request was received
#[derive(Debug, PartialEq, Eq, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actions() -> Result<()> {
        assert_eq!(Response::PingPong.action_byte(), 0);
        Ok(())
    }

    #[test]
    fn pingpong_encode() -> Result<()> {
        assert_eq!(Response::PingPong.to_msg()?, vec![0]);
        Ok(())
    }

    #[test]
    fn pingpong_decode() -> Result<()> {
        assert_eq!(Response::from_msg(vec![0])?, Response::PingPong);
        Ok(())
    }
}
