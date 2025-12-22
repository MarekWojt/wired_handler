use hyper::{StatusCode, body::Bytes};
use thiserror::Error;
#[cfg(feature = "diesel")]
use tracing::warn;

use super::response::Response;
use super::response_body::ResponseBody;
#[cfg(feature = "websocket")]
use super::send_message::BatchSendError;
use crate::prelude::*;

/// An HTTP error to be used when something goes wrong
#[derive(Debug, Error)]
#[error("{0:?}")]
pub struct HttpError(pub Response);

impl From<HttpError> for Response {
    fn from(value: HttpError) -> Self {
        value.0
    }
}

impl HttpError {
    /// Creates a new `HttpError` with the respective `StatusCode`
    pub fn new(status: StatusCode, message: impl Into<Bytes>) -> Self {
        Self(
            hyper::Response::builder()
                .status(status)
                .body(ResponseBody::from_bytes(message.into()))
                .unwrap(), // can only fail on headers, no headers given
        )
    }
}

impl From<Response> for HttpError {
    fn from(value: Response) -> Self {
        Self(value)
    }
}

impl From<http::Error> for HttpError {
    fn from(value: http::Error) -> Self {
        tracing::debug!("{value}");
        Self::internal_server_error("failed to create Response")
    }
}

#[cfg(feature = "diesel")]
impl From<diesel_async::pooled_connection::deadpool::PoolError> for HttpError {
    fn from(value: diesel_async::pooled_connection::deadpool::PoolError) -> Self {
        warn!("error getting database connection from pool: {value}");

        Self::internal_server_error("internal server error")
    }
}

#[cfg(feature = "diesel")]
impl From<diesel::result::Error> for HttpError {
    fn from(value: diesel::result::Error) -> Self {
        Self::internal_server_error(value.to_string())
    }
}

#[cfg(feature = "websocket")]
impl From<BatchSendError> for HttpError {
    fn from(value: BatchSendError) -> Self {
        HttpError::internal_server_error(format!("{value}"))
    }
}

/// Creates constructors on `HttpError`.
/// Accepted format is: `generate_constructors!((name1, StatusCode::SOME_STATUS), (name2, StatusCode::SOME_OTHER_STATUS))`
macro_rules! generate_constructors {
    (
        $(($name:ident, $status_code:expr),)*
    ) => {
        impl HttpError {
            $(
                /// Creates a new `HttpError` with the respective `StatusCode`
                pub fn $name(message: impl Into<Bytes>) -> Self {
                    Self (
                        hyper::Response::builder()
                            .status($status_code)
                            .body(ResponseBody::from_bytes(message.into()))
                            .unwrap(), // can only fail on headers, no headers given
                    )
                }
            )*
        }
    };
}

generate_constructors!(
    (not_found, StatusCode::NOT_FOUND),
    (forbidden, StatusCode::FORBIDDEN),
    (internal_server_error, StatusCode::INTERNAL_SERVER_ERROR),
    (bad_request, StatusCode::BAD_REQUEST),
    (unauthorized, StatusCode::UNAUTHORIZED),
    (not_implemented, StatusCode::NOT_IMPLEMENTED),
);
