use std::ops::ControlFlow;

use super::{http_error::HttpError, response_body::ResponseBody};
use crate::{
    prelude::*,
    state::{context::HttpRequestContext, request_state::RequestState},
};

pub type Response = hyper::Response<ResponseBody>;

/// Allows creating a `http::response::Builder` from types other than `hyper::Response::<()>`
pub trait ResponseBuilderExt {
    fn builder() -> http::response::Builder {
        hyper::Response::builder()
    }
}

impl ResponseBuilderExt for Response {}

/// Returns helpers for `HttpRequestContext`
pub trait ContextReturnResponseExt {
    /// Continue to run the next handler
    #[must_use = "Must be returned to be effective"]
    fn next(&mut self, response: Response) -> Result<ControlFlow<()>, HttpError>;

    /// Continue to run the next handler without returning a `Response`.
    /// **Only use if you know this isn't the last handler**
    #[must_use = "Must be returned to be effective"]
    fn next_no_response(&self) -> Result<ControlFlow<()>, HttpError> {
        Ok(ControlFlow::Continue(()))
    }

    /// Stops the execution, not running the next handler
    #[must_use = "Must be returned to be effective"]
    fn stop(&mut self, response: Response) -> Result<ControlFlow<()>, HttpError>;

    /// Stops the execution, not running the next handler, without returning a `Response`.
    /// **Only use if you know a `Response` is already inserted**
    #[must_use = "Must be returned to be effective"]
    fn stop_no_response(&self) -> Result<ControlFlow<()>, HttpError> {
        Ok(ControlFlow::Break(()))
    }
}

impl ContextReturnResponseExt for HttpRequestContext {
    fn next(&mut self, response: Response) -> Result<ControlFlow<()>, HttpError> {
        RequestState::get_mut_from_ctx(self).insert(response);
        Ok(ControlFlow::Continue(()))
    }

    fn stop(&mut self, response: Response) -> Result<ControlFlow<()>, HttpError> {
        RequestState::get_mut_from_ctx(self).insert(response);
        Ok(ControlFlow::Break(()))
    }
}

// TODO: test
