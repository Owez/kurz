//! Contains [Request] and implementations; see item-level docs for more info

use super::{Action, Message, MessageBytes, ToAction};
use crate::{Error, Result, Value};

/// Requests which are sent to other peers on the network, optionally expecting a response
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Request<K: Send, V: Send> {
    /// See [Action::PingPong]
    PingPong,
    /// See [Action::KeySend]
    KeySend((K, Value<V>)),
}

impl<K: Send, V: Send> ToAction for Request<K, V> {
    fn action(&self) -> Action {
        match self {
            Self::PingPong => Action::PingPong,
            Self::KeySend(_) => Action::KeySend,
        }
    }
}

impl<K: Send, V: Send> Message for Request<K, V> {
    fn from_msg(msg_bytes: MessageBytes) -> Result<Self> {
        // Verify length
        if msg_bytes.len() < 1 {
            return Err(Error::Length);
        }

        // Decode
        match Action::from_byte(msg_bytes[0])? {
            Action::PingPong => Ok(Self::PingPong),Action::KeySend=>todo!("decode key send")
        }
    }

    fn to_msg(&self) -> Result<MessageBytes> {
        Ok(match self {
            Self::PingPong => vec![self.action_byte()],
            Self::KeySend(_) => todo!("encode key send"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actions() -> Result<()> {
        let req: Request<(), ()> = Request::PingPong;
        assert_eq!(req.action_byte(), 0);
        Ok(())
    }

    #[test]
    fn pingpong_encode() -> Result<()> {
        let req: Request<(), ()> = Request::PingPong;
        assert_eq!(req.to_msg()?, vec![0]);
        Ok(())
    }

    #[test]
    fn pingpong_decode() -> Result<()> {
        let req: Request<(), ()> = Request::from_msg(vec![0])?;
        assert_eq!(req, Request::PingPong);
        Ok(())
    }
}
