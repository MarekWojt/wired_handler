use hyper::body::Incoming;

use crate::{
    prelude::*,
    state::{context::HttpRequestContext, request_state::RequestState},
};

pub type Request = hyper::Request<Incoming>;

/// Get the request from an `HttpRequestContext`
pub trait ContextGetRequestExt {
    fn request(&self) -> &Request;
    fn request_mut(&mut self) -> &mut Request;
}

impl ContextGetRequestExt for HttpRequestContext {
    fn request(&self) -> &Request {
        RequestState::get_from_ctx(self)
            .get::<Request>()
            .expect("every HttpRequestContext must have a Request")
    }

    fn request_mut(&mut self) -> &mut Request {
        RequestState::get_mut_from_ctx(self)
            .get_mut::<Request>()
            .expect("every HttpRequestContext must have a Request")
    }
}
