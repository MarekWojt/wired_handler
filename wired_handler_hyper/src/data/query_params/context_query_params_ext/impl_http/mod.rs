use serde::de::DeserializeOwned;

use crate::{
    data::query_params::{GetQueryParamsError, data::QueryParams},
    prelude::*,
    state::{context::HttpRequestContext, request_state::RequestState},
};

use super::ContextCreateQueryParamsExt;

#[cfg(test)]
mod test;

impl ContextCreateQueryParamsExt for HttpRequestContext {
    #[cfg(not(test))]
    fn raw_query_params(&self) -> Option<&str> {
        self.request().uri().query()
    }

    // Different implementation needed because we can't produce a Request<Incoming>
    #[cfg(test)]
    fn raw_query_params(&self) -> Option<&str> {
        use crate::state::request_state::RequestState;

        RequestState::get_from_ctx(self)
            .get::<String>()
            .map(String::as_str)
    }

    fn parse_query_params<T: DeserializeOwned + Send + Sync + 'static>(
        &self,
    ) -> Result<Option<T>, GetQueryParamsError> {
        let Some(query) = self.raw_query_params() else {
            return Ok(None);
        };
        let result: T = serde_html_form::from_str(query)?;

        Ok(Some(result))
    }

    fn insert_query_params<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<(), GetQueryParamsError> {
        let data = self.parse_query_params::<T>()?;

        RequestState::get_mut_from_ctx(self).insert(QueryParams::new(data));

        Ok(())
    }
}

impl ContextGetQueryParamsExt for HttpRequestContext {
    fn query_params<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<Option<&T>, GetQueryParamsError> {
        self.insert_query_params::<T>()?;

        Ok(RequestState::get_from_ctx(self)
            .get::<QueryParams<T>>()
            .unwrap() // just inserted
            .get())
    }

    fn query_params_mut<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<Option<&mut T>, GetQueryParamsError> {
        self.insert_query_params::<T>()?;

        Ok(RequestState::get_mut_from_ctx(self)
            .get_mut::<QueryParams<T>>()
            .unwrap() // just inserted
            .get_mut())
    }

    fn remove_query_params<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<Option<T>, GetQueryParamsError> {
        if let Some(query_params) =
            RequestState::get_mut_from_ctx(self).remove_get::<QueryParams<T>>()
        {
            return Ok(query_params.into_inner());
        }

        self.parse_query_params()
    }
}
