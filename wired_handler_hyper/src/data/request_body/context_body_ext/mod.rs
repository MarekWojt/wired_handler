use std::future::Future;

use hyper::body::Bytes;
use serde::de::DeserializeOwned;

use super::error::GetBodyError;

mod impl_http;
#[cfg(feature = "websocket")]
mod impl_websocket;

/// Helper trait for body handling
pub trait ContextCreateBodyExt {
    /// Whether a body of type `T` has been parsed
    fn body_exists<T: DeserializeOwned + Send + Sync + 'static>(&self) -> bool;

    /// Whether the body has been parsed
    fn is_body_parsed(&self) -> bool;

    /// Marks that the request's body has been parsed
    fn mark_body_parsed(&mut self);

    /// Returns all bytes of the body data, removing them from the request
    fn extract_body_bytes(&mut self) -> impl Future<Output = Result<Bytes, GetBodyError>>;

    /// Creates the body from the context
    fn create_body<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> impl Future<Output = Result<T, GetBodyError>> {
        async {
            let incoming_bytes = self.extract_body_bytes().await?;

            if incoming_bytes.is_empty() && self.is_body_parsed() {
                return Err(GetBodyError::AlreadyParsed);
            }

            self.mark_body_parsed();

            self.decode_data(&incoming_bytes)
        }
    }

    /// Turns the bytes into `T`
    fn decode_data<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
        bytes_to_decode: &[u8],
    ) -> Result<T, GetBodyError> {
        Ok(serde_json::from_slice(bytes_to_decode)?)
    }

    /// Inserts the body into the context
    fn insert_body<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> impl Future<Output = Result<(), GetBodyError>>;
}

/// Get a decoded body from an `HttpRequestContext`
pub trait ContextGetBodyExt: ContextCreateBodyExt {
    /// Parses and returns a reference to the body. The result is cached.
    ///
    /// The parsing can only be done once. Trying to get a different body type from the same request will result in an error
    fn body<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> impl Future<Output = Result<&T, GetBodyError>>;

    /// Parses and returns a mutable reference to the body. The result is cached.
    ///
    /// The parsing can only be done once. Trying to get a different body type from the same request will result in an error
    fn body_mut<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> impl Future<Output = Result<&mut T, GetBodyError>>;

    /// Parses and returns the body.
    ///
    /// *Use with caution, the body cannot be accessed after removing it!*
    fn remove_body<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> impl Future<Output = Result<T, GetBodyError>>;
}
