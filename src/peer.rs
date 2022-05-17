//! Contains [Peer] and implementations; see item-level docs for more info

use std::net::SocketAddr;

/// External peer on the network we can contact
pub struct Peer {
    /// Contact address
    pub addr: SocketAddr,
}
