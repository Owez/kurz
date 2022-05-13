/// Intention of any given message
pub enum Action {
    /// Allows peers to test connections to each other
    ///
    /// # About
    ///
    /// One peer sends an unsolicited ping-pong request another, and the receiving peer simply sends the ping-pong response back. This involves no data transfer but still uses encryption to test the full cycle works. The flow is the following:
    ///
    /// 1. Peer A sends this request to Peer B
    /// 2. Peer B decodes the request and sends back a ping-pong response
    /// 3. Peer A knows it can connect to Peer B if the response can be decrypted
    PingPong,
}

impl From<Action> for u8 {
    fn from(action: Action) -> Self {
        match action {
            Action::PingPong => 0,
        }
    }
}

/// Message conversion to it's [Action] intent
pub trait ToAction {
    /// Provides the full [Action] of what this message corresponds to
    fn action(&self) -> Action;

    /// Provides action byte of what the message corresponds to
    fn action_byte(&self) -> u8 {
        self.action().into()
    }
}
