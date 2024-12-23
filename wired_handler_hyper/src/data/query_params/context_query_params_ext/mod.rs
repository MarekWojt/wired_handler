use serde::de::DeserializeOwned;

use super::GetQueryParamsError;

mod impl_http;

pub trait ContextCreateQueryParamsExt {
    /// Gets raw query params
    fn raw_query_params(&self) -> Option<&str>;

    /// Parses query params and returns them
    fn parse_query_params<T: DeserializeOwned + Send + Sync + 'static>(
        &self,
    ) -> Result<Option<T>, GetQueryParamsError>;

    /// Parses query params and inserts them
    fn insert_query_params<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<(), GetQueryParamsError>;
}

pub trait ContextGetQueryParamsExt: ContextCreateQueryParamsExt {
    /// Parses and returns a reference to the query parameters. The result is cached
    fn query_params<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<Option<&T>, GetQueryParamsError>;

    /// Parses and returns a mutable reference to the query parameters. The result is cached
    fn query_params_mut<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<Option<&mut T>, GetQueryParamsError>;

    /// Parses and returns the query parameters. This returns the saved data from the cache (if present) and returns it without inserting anything new into it
    fn remove_query_params<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<Option<T>, GetQueryParamsError>;
}
