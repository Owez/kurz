use std::time::SystemTime;

/// Wrapper for a value with network metadata attached
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Value<V> {
    /// The actual value
    pub inner: V,
    /// Time this value was published to the system
    pub published: SystemTime,
}
