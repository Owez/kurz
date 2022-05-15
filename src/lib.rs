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

pub mod message;

mod errors;

pub use errors::{Error, Result};
