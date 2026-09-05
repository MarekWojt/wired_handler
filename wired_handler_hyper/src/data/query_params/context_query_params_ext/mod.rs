use serde::de::DeserializeOwned;

use super::GetQueryParamsError;

mod impl_http;

pub trait ContextQueryParamsCreateExt {
    /// Gets raw query params
    fn raw_query_params(&self) -> Option<&str>;

    /// Whether a successfully parsed query params of type `T` is cached
    fn query_params_cached_as<T: DeserializeOwned + Send + Sync + 'static>(&self) -> bool;

    /// Parses query params and returns them
    /// 
    /// # Errors
    /// Returns a `GetQueryParamsError` if parsing the query parameters fails
    fn parse_query_params<T: DeserializeOwned + Send + Sync + 'static>(
        &self,
    ) -> Result<Option<T>, GetQueryParamsError>;

    /// Parses query params and inserts them into the cache
    /// 
    /// # Errors
    /// Returns a `GetQueryParamsError` if parsing the query parameters fails
    fn cache_query_params<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<(), GetQueryParamsError>;
}

pub trait ContextQueryParamsGetExt: ContextQueryParamsCreateExt {
    /// Parses and returns a reference to the query parameters. The result is cached
    /// 
    /// # Errors
    /// Returns a `GetQueryParamsError` if parsing the query parameters fails
    fn query_params<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<Option<&T>, GetQueryParamsError>;

    /// Parses and returns a mutable reference to the query parameters. The result is cached
    /// 
    /// # Errors
    /// Returns a `GetQueryParamsError` if parsing the query parameters fails
    fn query_params_mut<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<Option<&mut T>, GetQueryParamsError>;

    /// Parses and returns the query parameters. This returns the saved data from the cache (if present) and returns it without inserting anything new into it
    /// 
    /// # Errors
    /// Returns a `GetQueryParamsError` if parsing the query parameters fails
    fn remove_query_params<T: DeserializeOwned + Send + Sync + 'static>(
        &mut self,
    ) -> Result<Option<T>, GetQueryParamsError>;
}
