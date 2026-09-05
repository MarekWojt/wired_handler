use thiserror::Error;

use crate::data::http_error::HttpError;

/// Error returned when failed to get the body
#[derive(Debug, Error)]
#[error("{0}")]
pub enum GetBodyError {
    #[cfg(feature = "json")]
    /// JSON decode error
    Json(#[from] serde_json::error::Error),
    /// Hyper error
    Hyper(#[from] hyper::Error),
    /// Failed to decode frame
    #[error("invalid frame data")]
    FailedFrame,
    /// Body has already been parsed into a different type or removed
    #[error("body has already been consumed or been removed")]
    BodyUnavailable,
    #[error("invalid message type")]
    InvalidMessageType,
}

impl From<GetBodyError> for HttpError {
    fn from(value: GetBodyError) -> Self {
        match value {
            #[cfg(feature = "json")]
            GetBodyError::Json(json_error) => Self::bad_request(json_error.to_string()),
            GetBodyError::Hyper(hyper_error) => {
                tracing::debug!("Hyper error handling body frame: {:?}", hyper_error);
                Self::internal_server_error("Hyper error handling body")
            }
            GetBodyError::FailedFrame => Self::bad_request("a frame couldn't be converted"),
            GetBodyError::BodyUnavailable => {
                Self::internal_server_error("body has already been consumed or been removed")
            }
            GetBodyError::InvalidMessageType => {
                Self::internal_server_error("internal server error")
            }
        }
    }
}
