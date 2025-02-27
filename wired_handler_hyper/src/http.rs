use std::{future::Future, net::SocketAddr};

use hyper::{
    service::{service_fn, Service},
    StatusCode,
};
use hyper_util::rt::TokioTimer;
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, trace};
use wired_handler::Handler;

use crate::{
    data::{config::BindConfig, request::Request, response::Response, response_body::ResponseBody},
    prelude::*,
    state::{
        context::{
            HttpRequestContext, SessionlessRequestContext, SessionlessRequestContextBuilder,
        },
        global_state::GlobalState,
        request_state::RequestState,
    },
};

/// Handles a connection, can be a single HTTP request or multiple if keep-alive is used
async fn handle_connection(
    stream: TcpStream,
    _addr: SocketAddr,
    http_service_fn: impl Service<Request, Error = hyper::http::Error, Response = Response>,
) -> Result<(), hyper::Error> {
    let io = hyper_util::rt::TokioIo::new(stream);

    let mut http_builder = hyper::server::conn::http1::Builder::new();
    let conn = http_builder
        .timer(TokioTimer::new())
        .serve_connection(io, http_service_fn);

    #[cfg(feature = "websocket")]
    conn.with_upgrades().await?;

    #[cfg(not(feature = "websocket"))]
    conn.await?;

    Ok(())
}

/// Handles a single request, turning a `Request` in a `Response`
async fn handle_request(
    request: Request,
    handler: Handler<
        SessionlessRequestContext,
        HttpRequestContext,
        GlobalState,
        impl Future<Output = HttpRequestContext> + 'static + Send,
    >,
) -> Result<Response, hyper::http::Error> {
    let request_state = {
        let mut request_state = RequestState::default();
        request_state.insert(request);
        request_state
    };

    let mut result_ctx = handler
        .handle(SessionlessRequestContextBuilder { request_state })
        .await;

    let response: Option<Response> = RequestState::get_mut_from_ctx(&mut result_ctx).remove_get();

    Ok(response.unwrap_or_else(|| {
        hyper::Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(ResponseBody::from_bytes("missing response"))
            .unwrap() // This is static code, it either always fails or never
    }))
}

#[derive(Debug, Error)]
#[error("{0}")]
pub enum RunHttpServerError {
    Hyper(#[from] hyper::Error),
    Io(#[from] std::io::Error),
}

pub trait RunHttpServerExt {
    /// Runs the HTTP server with the given `bind_config`
    fn run_http_server(
        self,
        bind_config: BindConfig,
    ) -> impl Future<Output = Result<(), RunHttpServerError>>;
}

impl<F: Future<Output = HttpRequestContext> + 'static + Send> RunHttpServerExt
    for Handler<SessionlessRequestContext, HttpRequestContext, GlobalState, F>
{
    async fn run_http_server(self, bind_config: BindConfig) -> Result<(), RunHttpServerError> {
        let bind_addr = bind_config.addr.as_str();

        info!("starting http server on {bind_addr}");
        let tcp_listener = TcpListener::bind(bind_addr).await?;
        info!("listening on http://{bind_addr}");

        self.state().insert(bind_config).await;

        let http_service_fn =
            service_fn(move |request: Request| handle_request(request, self.clone()));
        loop {
            let (stream, addr) = tcp_listener.accept().await?;
            trace!("new connection on {:?}", &addr);
            tokio::spawn(handle_connection(stream, addr, http_service_fn.clone()));
        }
    }
}
