use http_body_util::{BodyExt, Empty, Full, combinators::BoxBody};
use hyper::body::Bytes;
use serde::Serialize;
use thiserror::Error;

use crate::{
    data::{http_error::HttpError, response::Response},
    state::context::HttpRequestContext,
};

pub type ResponseBody = BoxBody<hyper::body::Bytes, hyper::Error>;

/// For quickly creating a `ResponseBody`
pub trait ResponseBodyExt {
    fn from_bytes(bytes_body: impl Into<Bytes>) -> Self;
    fn empty() -> Self;
}

impl ResponseBodyExt for ResponseBody {
    fn from_bytes(bytes_body: impl Into<Bytes>) -> Self {
        Full::new(bytes_body.into())
            .map_err(|never| match never {})
            .boxed()
    }

    fn empty() -> Self {
        Empty::new().map_err(|never| match never {}).boxed()
    }
}

/// Lets you add a parsed body to a request builder
pub trait ResponseBuilderParsedBodyExt {
    /// Creates a `Response` by applying a `ParsedBody`
    fn parsed_body(self, body: ParsedBody) -> Result<Response, http::Error>;
}

impl ResponseBuilderParsedBodyExt for http::response::Builder {
    fn parsed_body(self, body: ParsedBody) -> Result<Response, http::Error> {
        let response = self
            .header("Content-Type", body.content_type.as_str())
            .body(body.response_body)?;

        Ok(response)
    }
}

/// When parsing the body goes wrong
#[derive(Debug, Error)]
#[error("{0}")]
pub enum ParseBodyError {
    #[cfg(feature = "json")]
    Json(#[from] serde_json::Error),
}
impl From<ParseBodyError> for HttpError {
    fn from(value: ParseBodyError) -> Self {
        tracing::debug!("parse error: {value}");
        Self::internal_server_error("failed to parse response body")
    }
}

/// A parsed body, containing the body and type information to be inserted into the Response
#[derive(Debug)]
pub struct ParsedBody {
    response_body: ResponseBody,
    content_type: String,
}

/// For parsing a body from data
pub trait CtxParseBodyExt {
    /// Parses `data` into a `ParsedBody`
    fn parse_body<T: Serialize>(&self, data: T) -> Result<ParsedBody, ParseBodyError>;
}

impl CtxParseBodyExt for HttpRequestContext {
    fn parse_body<T: Serialize>(&self, data: T) -> Result<ParsedBody, ParseBodyError> {
        let parsed_data = serde_json::to_vec(&data)?;
        Ok(ParsedBody {
            response_body: ResponseBody::from_bytes(parsed_data),
            content_type: "application/json".to_string(),
        })
    }
}
