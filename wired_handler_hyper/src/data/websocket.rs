use std::{future::Future, ops::ControlFlow};

use async_fn_traits::AsyncFn1;
use futures::StreamExt;
use http::StatusCode;
use http_body_util::BodyExt;

use super::{
    connection_id::ConnectionId, connection_storage::ConnectionStorage, http_error::HttpError,
    response::Response, response_body::ResponseBody,
};
use crate::{
    prelude::*,
    state::{
        connection_state::ConnectionState,
        context::{HttpRequestContext, WebsocketRequestContext},
        global_state::GlobalState,
        request_state::RequestState,
        session_state::SessionState,
    },
};

/// For upgrading the request to a websocket
pub trait ContextWebsocketExt {
    fn next_websocket<
        Fn: Send + 'static + for<'a> AsyncFn1<&'a mut WebsocketRequestContext, Output = ()>,
    >(
        &mut self,
        handler_fn: Fn,
    ) -> impl Future<Output = Result<ControlFlow<()>, HttpError>>
    where
        for<'a> <Fn as AsyncFn1<&'a mut WebsocketRequestContext>>::OutputFuture: Send;
}

impl ContextWebsocketExt for HttpRequestContext {
    /// Handles upgrading to websocket, using `handler_fn` as its handler. Inserts a `Response`
    async fn next_websocket<
        Fn: Send + 'static + for<'a> AsyncFn1<&'a mut WebsocketRequestContext, Output = ()>,
    >(
        &mut self,
        handler_fn: Fn,
    ) -> Result<ControlFlow<()>, HttpError>
    where
        for<'a> <Fn as AsyncFn1<&'a mut WebsocketRequestContext>>::OutputFuture: Send,
    {
        // return error if request isn't websocket request
        if !hyper_tungstenite::is_upgrade_request(self.request()) {
            return Err(HttpError::from(
                Response::builder()
                    .status(StatusCode::UPGRADE_REQUIRED)
                    .header("Upgrade", "websocket")
                    .body(ResponseBody::empty())?,
            ));
        }

        // upgrade to websocket
        let (response, websocket) = hyper_tungstenite::upgrade(self.request_mut(), None)
            .map_err(|err| HttpError::new(StatusCode::BAD_REQUEST, err.to_string()))?;

        // clone (semi-)global states
        let session_state = SessionState::get_from_ctx(self).clone();
        let global_state = GlobalState::get_from_ctx(self).clone();

        // spin up task for message handling
        tokio::spawn(async move {
            let (connection_state, connection_id, mut rx) = {
                // split sink (for sending) and stream (for receiving)
                let (tx, rx) = {
                    let websocket = match websocket.await {
                        Ok(ws) => ws,
                        Err(err) => {
                            tracing::debug!("failed to initialize websocket connection: {err}");
                            return;
                        }
                    };
                    websocket.split()
                };

                let connection_id = ConnectionId::generate();

                // create ConnectionState from tx and connection id
                let connection_state = {
                    let connection_state = ConnectionState::default();
                    connection_state.insert(tx).await;
                    connection_state.insert(connection_id).await;
                    connection_state
                };

                // insert connection state into session
                {
                    let mut connection_storage = session_state
                        .get_mut_or_insert_default::<ConnectionStorage>()
                        .await;
                    connection_storage
                        .get_mut()
                        .insert(connection_id, connection_state.clone());
                }

                (connection_state, connection_id, rx)
            };

            let mut ctx = WebsocketRequestContext::from_states(
                global_state,
                session_state.clone(),
                connection_state.clone(),
                RequestState::default(),
            );

            // message handling
            while let Some(message) = rx.next().await {
                use hyper_tungstenite::tungstenite::Error as TungsteniteError;
                use std::io::ErrorKind as IoErrorKind;

                // output errors
                let message = match message {
                    Ok(message) => message,
                    // abort on fatal errors
                    Err(err)
                        if matches!(
                            err,
                            TungsteniteError::AttackAttempt
                                | TungsteniteError::Protocol(_)
                                | TungsteniteError::Tls(_)
                                | TungsteniteError::ConnectionClosed
                                | TungsteniteError::AlreadyClosed
                        ) =>
                    {
                        tracing::debug!("websocket receive failed, aborting: {err}");
                        break;
                    }
                    // abort on io errors except WouldBlock
                    Err(TungsteniteError::Io(err))
                        if !matches!(err.kind(), IoErrorKind::WouldBlock) =>
                    {
                        tracing::debug!("websocket receive failed, aborting: {err}");
                        break;
                    }
                    Err(err) => {
                        tracing::debug!("websocket receive failed, skipping: {err}");
                        continue;
                    }
                };

                let request_state = {
                    let mut request_state = RequestState::default();
                    request_state.insert(message);
                    request_state
                };

                *RequestState::get_mut_from_ctx(&mut ctx) = request_state;

                handler_fn(&mut ctx).await;
            }

            // remove connection state from session
            session_state
                .get_mut_or_insert_default::<ConnectionStorage>()
                .await
                .get_mut()
                .remove(&connection_id);
        });

        // convert response to the correct type
        let converted_response = {
            let (response_parts, response_body) = response.into_parts();
            Response::from_parts(
                response_parts,
                response_body.map_err(|err| match err {}).boxed(),
            )
        };

        self.next(converted_response)
    }
}
