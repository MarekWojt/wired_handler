use hyper::body::Bytes;
use serde::de::DeserializeOwned;

use super::ContextBodyCreateExt;
use crate::{
    data::request_body::{
        data::{RequestBody, RequestBodyConsumed},
        error::GetBodyError,
    },
    prelude::*,
    state::{context::WebsocketRequestContext, request_state::RequestState},
};

impl ContextBodyCreateExt for WebsocketRequestContext {
    fn is_body_consumed(&self) -> bool {
        RequestState::get_from_ctx(self).exists::<RequestBodyConsumed>()
    }

    fn mark_body_consumed(&mut self) {
        RequestState::get_mut_from_ctx(self).insert(RequestBodyConsumed);
    }

    fn body_cached_as<T: DeserializeOwned + Send + Sync + 'static>(&self) -> bool {
        RequestState::get_from_ctx(self).exists::<RequestBody<T>>()
    }

    fn take_body_bytes(&mut self) -> impl Future<Output = Result<Bytes, GetBodyError>> {
        use std::future::ready;
        use hyper_tungstenite::tungstenite::Message;

        let message = self.message_mut();
        let collected_bytes = match message {
            Message::Text(data) => Bytes::from(std::mem::take(data)),
            Message::Binary(data) => std::mem::take(data),
            _ => return ready(Err(GetBodyError::InvalidMessageType)),
        };

        ready(Ok(collected_bytes))
    }

    async fn cache_body<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<(), GetBodyError> {
        let body = self.create_body::<T>().await?;
        let request_state = RequestState::get_mut_from_ctx(self);
        request_state.insert(RequestBody::new(body));

        Ok(())
    }
}

impl ContextBodyGetExt for WebsocketRequestContext {
    async fn body<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<&T, GetBodyError> {
        if !self.body_cached_as::<T>() {
            self.cache_body::<T>().await?;
        }

        Ok(
            RequestState::get_from_ctx(self)
                .get::<RequestBody<T>>()
                .map(RequestBody::get)
                .unwrap(), // has just been inserted
        )
    }

    async fn body_mut<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<&mut T, GetBodyError> {
        if !self.body_cached_as::<T>() {
            self.cache_body::<T>().await?;
        }

        Ok(
            RequestState::get_mut_from_ctx(self)
                .get_mut::<RequestBody<T>>()
                .map(RequestBody::get_mut)
                .unwrap(), // has just been inserted
        )
    }

    async fn remove_body<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<T, GetBodyError> {
        if self.body_cached_as::<T>() {
            return Ok(RequestState::get_mut_from_ctx(self)
                .remove_get::<RequestBody<T>>()
                .unwrap()
                .into_inner());
        }

        self.create_body().await
    }
}

#[cfg(test)]
mod test;
