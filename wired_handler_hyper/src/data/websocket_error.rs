use thiserror::Error;

use super::send_message::BatchSendError;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct WebsocketError(pub String);

impl WebsocketError {
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl From<BatchSendError> for WebsocketError {
    fn from(value: BatchSendError) -> Self {
        WebsocketError(format!("{value}"))
    }
}
