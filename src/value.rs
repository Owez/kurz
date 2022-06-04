use std::time::SystemTime;

use bincode::{Decode, Encode};

/// Wrapper for a value with network metadata attached
#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
pub struct Value<V: Send + Encode + Decode> {
    /// The actual value
    pub inner: V,
    /// Time this value was published to the system
    pub published: SystemTime,
}
