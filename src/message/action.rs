/// Intention of any given message
pub enum Action {}

impl From<Action> for u8 {
    fn from(action: Action) -> Self {
        match action {}
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
