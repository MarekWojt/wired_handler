use std::{
    future::Future,
    sync::{Arc, LazyLock},
};

use futures::stream::{FuturesUnordered, StreamExt};
use hyper_tungstenite::tungstenite::Error as TungsteniteError;
use hyper_tungstenite::tungstenite::Message;
use tokio::sync::Semaphore;
use wired_handler::{Context, GetState};

use super::{BatchSendError, SendMessageExt, SingleSendError};
use crate::{
    data::{
        connection_storage::ConnectionStorage,
        send_message::{SendMessageError, global_max_parallel_sends},
    },
    prelude::*,
    state::session_state::SessionState,
};

static GLOBAL_SEND_SEMAPHORE: LazyLock<Semaphore> =
    LazyLock::new(|| Semaphore::new(global_max_parallel_sends()));

/// For sending messages to a context
pub trait ContextSendMessageExt {
    /// Sends a Message to all connections of the current session
    ///
    /// Returns the count of connections the message was sent to, or an error consisting of a list of all errors
    fn send_message(&self, message: Message)
    -> impl Future<Output = Result<usize, BatchSendError>>;
}

// impl for any `Context` that contains a `SessionState`
impl<T: Context> ContextSendMessageExt for T
where
    SessionState: GetState<T>,
{
    async fn send_message(&self, message: Message) -> Result<usize, BatchSendError> {
        // get connection storage from session
        let Some(connection_storage) = SessionState::get_from_ctx(self)
            .get::<ConnectionStorage>()
            .await
        else {
            return Ok(0); // no connections, nothing to send
        };

        let mut failures = Vec::new();

        // start sending
        let mut futures = FuturesUnordered::new();

        let message_arc = Arc::new(message);

        for (&connection_id, connection_state) in connection_storage.get() {
            let cloned_message = message_arc.clone();
            let cloned_connection_state = connection_state.clone();
            futures.push(async move {
                let _permit = GLOBAL_SEND_SEMAPHORE
                    .acquire()
                    .await
                    .expect("global send semaphore poisoned");
                (
                    connection_id,
                    cloned_connection_state
                        .send_message((*cloned_message).clone())
                        .await,
                )
            });
        }

        // collect send results
        let mut success_count = 0;
        while let Some((connection_id, task)) = futures.next().await {
            match task {
                // closed connections are automatically removed => no-op
                Err(SendMessageError::Tungstenite(TungsteniteError::ConnectionClosed)) => {}
                // closed connections should automatically be removed => bug
                Err(SendMessageError::Tungstenite(TungsteniteError::AlreadyClosed)) => {
                    tracing::debug!(
                        "connection {connection_id:?} already closed, please check if connections are deleted properly when closed"
                    );
                }
                Ok(()) => success_count += 1,
                Err(error) => failures.push(SingleSendError::new(error, connection_id)),
            }
        }

        if !failures.is_empty() {
            return Err(BatchSendError::new(failures, success_count));
        }

        Ok(success_count)
    }
}

// TODO: Tests
