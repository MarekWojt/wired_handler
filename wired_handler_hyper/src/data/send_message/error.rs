use std::fmt::Display;

use thiserror::Error;
use tokio::time::error::Elapsed;

use crate::data::connection_id::ConnectionId;

/// A collection of errors, containing all and failed sends and the count of successful sends
#[derive(Debug, Error)]
pub struct BatchSendError {
    failures: Box<[SingleSendError]>,
    success_count: usize,
}

impl Display for BatchSendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} of {} sends failed:",
            self.failure_count(),
            self.total_count()
        )?;

        for err in self.failures() {
            writeln!(f, "  connection {}: {}", err.connection_id(), err.error())?;
        }

        Ok(())
    }
}

impl BatchSendError {
    pub fn new(failures: impl Into<Box<[SingleSendError]>>, success_count: usize) -> Self {
        Self {
            failures: failures.into(),
            success_count,
        }
    }

    pub fn failures(&self) -> &[SingleSendError] {
        &self.failures
    }

    pub fn failure_count(&self) -> usize {
        self.failures.len()
    }

    pub fn success_count(&self) -> usize {
        self.success_count
    }

    pub fn total_count(&self) -> usize {
        self.success_count() + self.failure_count()
    }
}

/// A send error of a single message
#[derive(Debug, Error)]
#[error("{0}")]
pub enum SendMessageError {
    Tungstenite(#[from] hyper_tungstenite::tungstenite::Error),
    Timeout(#[from] Elapsed),
}

/// A single failed send error, consisting of a `ConnectionId` and a `SendMessageError`
#[derive(Debug, Error)]
#[error("{error}")]
pub struct SingleSendError {
    #[source]
    error: SendMessageError,
    connection_id: ConnectionId,
}

impl SingleSendError {
    pub fn new(error: SendMessageError, connection_id: ConnectionId) -> Self {
        Self {
            error,
            connection_id,
        }
    }

    pub fn error(&self) -> &SendMessageError {
        &self.error
    }

    pub fn into_error(self) -> SendMessageError {
        self.error
    }

    pub fn connection_id(&self) -> ConnectionId {
        self.connection_id
    }
}
