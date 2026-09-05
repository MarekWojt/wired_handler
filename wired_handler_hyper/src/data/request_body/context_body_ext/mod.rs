use std::future::Future;

use hyper::body::Bytes;
use serde::de::DeserializeOwned;

use super::error::GetBodyError;

mod impl_http;
#[cfg(feature = "websocket")]
mod impl_websocket;

/// Helper trait for body handling
pub trait ContextBodyCreateExt {
    /// Whether a successfully parsed body of type `T` is cached
    fn body_cached_as<T: DeserializeOwned + Send + Sync + 'static>(&self) -> bool;

    /// Whether the body has been parsed
    fn is_body_consumed(&self) -> bool;

    /// Marks that the request's body has been parsed
    fn mark_body_consumed(&mut self);

    /// Returns all bytes of the body data, removing them from the request
    fn take_body_bytes(&mut self) -> impl Future<Output = Result<Bytes, GetBodyError>>;

    /// Creates the body from the context.
    ///
    /// Consumes the request body. The body can only be created once.
    ///
    /// # Errors
    /// Errors if creating the body fails
    fn create_body<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> impl Future<Output = Result<T, GetBodyError>> {
        async {
            let incoming_bytes = self.take_body_bytes().await?;

            if incoming_bytes.is_empty() && self.is_body_consumed() {
                return Err(GetBodyError::BodyUnavailable);
            }

            self.mark_body_consumed();

            self.decode_data(&incoming_bytes)
        }
    }

    /// Turns the bytes into `T`
    ///
    /// # Errors
    /// Errors if decoding fails, returning an error of the type of the decoding (e.g. `GetBodyError::Json` for JSON)
    fn decode_data<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
        bytes_to_decode: &[u8],
    ) -> Result<T, GetBodyError> {
        Ok(
            #[cfg(feature = "json")]
            serde_json::from_slice(bytes_to_decode)?,
        )
    }

    /// Inserts the body into the context
    fn cache_body<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> impl Future<Output = Result<(), GetBodyError>>;
}

/// Get a decoded body from an `HttpRequestContext`
pub trait ContextBodyGetExt: ContextBodyCreateExt {
    /// Parses and returns a reference to the body of the request. The result is cached.
    ///
    /// The parsing can only be done once. Trying to get a different body type from the same request will result in an error
    fn body<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> impl Future<Output = Result<&T, GetBodyError>>;

    /// Parses and returns a mutable reference to the body of the request. The result is cached.
    ///
    /// The parsing can only be done once. Trying to get a different body type from the same request will result in an error
    fn body_mut<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> impl Future<Output = Result<&mut T, GetBodyError>>;

    /// Returns the body of the request, removing it from the context.
    ///
    /// *Use with care, the body cannot be accessed after removing it!*
    fn remove_body<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> impl Future<Output = Result<T, GetBodyError>>;
}
