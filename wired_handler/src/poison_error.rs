use std::sync::PoisonError as SyncPoisonError;
use thiserror::Error;

/// For returning a `std::sync::PoisonError` without having to provide any data
#[derive(Debug, Error)]
#[error("Thread has been poisoned")]
pub struct PoisonError;

impl<T> From<SyncPoisonError<T>> for PoisonError {
    fn from(_: SyncPoisonError<T>) -> Self {
        Self
    }
}
