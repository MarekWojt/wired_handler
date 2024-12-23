use hyper::body::Bytes;
use serde::de::DeserializeOwned;

use super::ContextCreateBodyExt;
use crate::{
    data::request_body::{
        data::{RequestBody, RequestBodyParsed},
        error::GetBodyError,
    },
    prelude::*,
    state::{context::WebsocketRequestContext, request_state::RequestState},
};

impl ContextCreateBodyExt for WebsocketRequestContext {
    fn is_body_parsed(&self) -> bool {
        RequestState::get_from_ctx(self).exists::<RequestBodyParsed>()
    }

    fn mark_body_parsed(&mut self) {
        RequestState::get_mut_from_ctx(self).insert(RequestBodyParsed)
    }

    fn body_exists<T: DeserializeOwned + Send + Sync + 'static>(&self) -> bool {
        RequestState::get_from_ctx(self).exists::<RequestBody<T>>()
    }

    async fn extract_body_bytes(&mut self) -> Result<Bytes, GetBodyError> {
        use hyper_tungstenite::tungstenite::Message;

        let message = self.message_mut();
        let collected_bytes = match message {
            Message::Text(data) => Bytes::from(std::mem::take(data)),
            Message::Binary(data) => std::mem::take(data),
            _ => return Err(GetBodyError::InvalidMessageType),
        };

        Ok(collected_bytes)
    }

    async fn insert_body<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<(), GetBodyError> {
        let body = self.create_body::<T>().await?;
        let request_state = RequestState::get_mut_from_ctx(self);
        request_state.insert(RequestBody::new(body));

        Ok(())
    }
}

impl ContextGetBodyExt for WebsocketRequestContext {
    async fn body<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<&T, GetBodyError> {
        if !self.body_exists::<T>() {
            self.insert_body::<T>().await?;
        }

        Ok(
            RequestState::get_from_ctx(self)
                .get::<RequestBody<T>>()
                .map(|request_body| request_body.get())
                .unwrap(), // has just been inserted
        )
    }

    async fn body_mut<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<&mut T, GetBodyError> {
        if !self.body_exists::<T>() {
            self.insert_body::<T>().await?;
        }

        Ok(
            RequestState::get_mut_from_ctx(self)
                .get_mut::<RequestBody<T>>()
                .map(|request_body| request_body.get_mut())
                .unwrap(), // has just been inserted
        )
    }

    async fn remove_body<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<T, GetBodyError> {
        if self.body_exists::<T>() {
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
