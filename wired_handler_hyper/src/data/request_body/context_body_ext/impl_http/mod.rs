#[cfg(not(test))]
use http_body_util::BodyExt;

use hyper::body::Bytes;
use serde::de::DeserializeOwned;

use super::ContextBodyCreateExt;
use crate::{
    data::request_body::{
        data::{RequestBody, RequestBodyConsumed},
        error::GetBodyError,
    },
    prelude::*,
    state::{context::HttpRequestContext, request_state::RequestState},
};

impl ContextBodyCreateExt for HttpRequestContext {
    fn is_body_consumed(&self) -> bool {
        RequestState::get_from_ctx(self).exists::<RequestBodyConsumed>()
    }

    fn mark_body_consumed(&mut self) {
        RequestState::get_mut_from_ctx(self).insert(RequestBodyConsumed);
    }

    fn body_cached_as<T: DeserializeOwned + Send + Sync + 'static>(&self) -> bool {
        RequestState::get_from_ctx(self).exists::<RequestBody<T>>()
    }

    // Different implementation needed because we can't produce a Request<Incoming>
    #[cfg(test)]
    async fn take_body_bytes(&mut self) -> Result<Bytes, GetBodyError> {
        Ok(RequestState::get_mut_from_ctx(self)
            .remove_get::<Bytes>()
            .unwrap_or_else(Bytes::new))
    }

    #[cfg(not(test))]
    async fn take_body_bytes(&mut self) -> Result<Bytes, GetBodyError> {
        let request = self.request_mut();
        let incoming = request.body_mut();
        let mut collected_bytes = Vec::new();
        while let Some(next_frame) = incoming.frame().await {
            let next_frame = next_frame?
                .into_data()
                .map_err(|_| GetBodyError::FailedFrame)?;

            collected_bytes.extend(next_frame);
        }
        Ok(collected_bytes.into())
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

impl ContextBodyGetExt for HttpRequestContext {
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
