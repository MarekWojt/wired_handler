use thiserror::Error;
use tokio::task::JoinError;

use crate::data::connection_id::ConnectionId;

/// A collection of errors, containing all successful and failed sends
#[derive(Debug, Error)]
#[error("{0:?}")]
pub struct BatchSendError(pub Vec<Result<(), SingleSendError>>);

/// The actual send error
#[derive(Debug, Error)]
#[error("{0}")]
pub enum SingleSendInnerError {
    Tungstenite(#[from] hyper_tungstenite::tungstenite::Error),
    Join(#[from] JoinError),
}

/// A single failed send error, consisting of a `ConnectionId` and a `SingleSendInnerError`
#[derive(Debug, Error)]
#[error("{error}")]
pub struct SingleSendError {
    error: SingleSendInnerError,
    connection_id: ConnectionId,
}

impl SingleSendError {
    pub fn new(error: SingleSendInnerError, connection_id: ConnectionId) -> Self {
        Self {
            error,
            connection_id,
        }
    }

    pub fn error(&self) -> &SingleSendInnerError {
        &self.error
    }

    pub fn into_error(self) -> SingleSendInnerError {
        self.error
    }

    pub fn connection_id(&self) -> ConnectionId {
        self.connection_id
    }
}
