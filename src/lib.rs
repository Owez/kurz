//! Distributed key-value store based on gossip networking
//!
//! # Specification
//!
//! ## Messaging
//!
//! There are two main types of messages which can be sent from one peer to another:
//! - [Request](message::Request): One peer asking/telling another peer some info
//! - [Response](message::Response): The other peer optionally replying back with some info
//!
//! Both of these messages are used exclusively via the [Message](message::Message) trait, which makes them symmetrical in nature; meaning you can decode a response and resend the response to another peer.
//!
//! ## Actions
//!
//! To differentiate different requests and responses from each other, a simple byte is affixed to the start of the message, this is called the [Action](message::Action) byte. Under the hood, requests and responses are just enums which mirror every action with optionally some extra message data.
//!
//! ## Encryption
//!
//! All messages, no matter how big or small, are encrypted. This is done using [AES256-GCM-SIV](aes_gcm_siv) with packets being layed out like so:
//!
//! 1. 92 bytes of nonce
//! 2. Encrypted message data
//!
//! Because every network operating with kurz is private, encryption is extremely simple. Each peer is passed the network's encryption key at startup, which should be stored somewhere safe. This secret network encryption key is known across all peers and is used to fully [authenticate](https://auth0.com/docs/get-started/identity-fundamentals/authentication-and-authorization) any peer.
// TODO: kurz and peer docs

pub mod message;

mod errors;
mod peer;
mod value;

pub use errors::{Error, Result};
pub use peer::Peer;
pub use value::Value;

use crate::message::PacketBytes;
use bincode::{Decode, Encode};
use log::{trace, warn};
use message::{Key, Message, Request, Response};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{net::UdpSocket, sync::Mutex};

/// Peer list wrapped in an arc mutex, allowing lenient borrowing
pub(crate) type Peers = Arc<Mutex<Vec<Peer>>>;

/// UDP-based socket wrapped in an arc for lenient borrowing
pub(crate) type Socket = Arc<UdpSocket>;

/// Store containing hashmap of wrapped values
pub(crate) type Store<K, V> = Arc<Mutex<HashMap<K, Value<V>>>>;

/// Representation of ourself on the network
pub struct Kurz<K: Send + Encode + Decode, V: Send + Encode + Decode> {
    /// Socket for communication
    pub socket: Socket,
    /// Encryption key
    pub key: Key,
    /// Peers we know of
    pub peers: Peers,
    /// Key-value store
    pub store: Store<K, V>,
}

impl<K: Send + Encode + Decode, V: Send + Encode + Decode> Kurz<K, V> {
    /// Creates new kurz instance locked with a `key` and assigned to the default address
    pub async fn new(key: &[u8; 32]) -> Result<Self> {
        Self::new_custom("0.0.0.0:7667".parse().unwrap(), key).await
    }

    /// Creates new kurz instance assigned to a custom address and locked with the `key` provided
    pub async fn new_custom(addr: SocketAddr, key: &[u8; 32]) -> Result<Self> {
        let socket = UdpSocket::bind(addr)
            .await
            .map_err(|err| Error::Bind(err))?;

        Ok(Self {
            socket: Arc::new(socket),
            key: Key::new(key),
            peers: Arc::new(Mutex::new(vec![])),
            store: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Listens and responds to network requests
    pub async fn listen(&self) {
        /// 1KiB maximum message length
        const MAX_BUF: usize = 1024;

        let mut buf = [0; MAX_BUF];

        // Get values
        let socket = Arc::clone(&self.socket);
        let key = self.key.clone();
        let peers = Arc::clone(&self.peers);

        // Listen handler
        tokio::spawn(async move {
            loop {
                // Get packet and address
                let (len, addr) = match socket.recv_from(&mut buf).await {
                    Ok(val) => val,
                    Err(err) => {
                        warn!("Couldn't receive message, {}", err);
                        continue;
                    }
                };

                // Ensure length isn't too large
                if len > MAX_BUF {
                    trace!("Provided message length was too long");
                    continue;
                }

                // Get values
                let socket = Arc::clone(&socket);
                let key = key.clone();
                let peers = Arc::clone(&peers);
                let packet: PacketBytes = buf[..len].to_vec();

                // Spin up handler
                tokio::spawn(async move {
                    match Self::listen_handle(socket, key, Arc::clone(&peers), addr, packet).await {
                        Ok(_) => (),
                        Err(err) => {
                            trace!("Error whilst handling message, {}", err);
                        }
                    }
                });
            }
        });
    }

    /// Sends debug `msg` to provided `peer` regardless of validity
    pub async fn send_debug(&self, peer: &Peer, msg: impl Message) -> Result<()> {
        Self::send_static(&self.socket, &self.key, peer.addr, msg).await
    }

    /// Handles a packet provided to [Self::listen] asynchronously
    async fn listen_handle(
        socket: Socket,
        key: Key,
        peers: Peers,
        // TODO: get store here
        addr: SocketAddr,
        packet: PacketBytes,
    ) -> Result<()> {
        let req: Request<K, V> = Request::from_packet(&key, packet)?;
        match req {
            Request::PingPong => Self::send_static(&socket, &key, addr, Response::PingPong).await,
            Request::KeySend((k, value)) => todo!("handle incoming key send"),
        }
    }

    /// Sends provided `msg` to `addr` statically
    async fn send_static(
        socket: &UdpSocket,
        key: &Key,
        addr: SocketAddr,
        msg: impl Message,
    ) -> Result<()> {
        let packet = msg.to_packet(key)?;
        socket
            .send_to(&packet[..], addr)
            .await
            .map_err(|err| Error::Send(err))?;
        Ok(())
    }
}
