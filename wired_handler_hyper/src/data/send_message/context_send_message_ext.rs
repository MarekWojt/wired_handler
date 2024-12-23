use std::future::Future;

use hyper_tungstenite::tungstenite::Message;
use wired_handler::{Context, GetState};

use super::{BatchSendError, SendMessageExt, SingleSendError};
use crate::{
    data::connection_storage::ConnectionStorage, prelude::*, state::session_state::SessionState,
};

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
        let connection_storage = match SessionState::get_from_ctx(self)
            .get::<ConnectionStorage>()
            .await
        {
            Some(connection_storage) => connection_storage,
            None => return Ok(0), // no connections, nothing to send
        };

        // start sending
        let mut tasks = Vec::new();
        for (&connection_id, connection_state) in connection_storage.get() {
            let cloned_message = message.clone();
            let cloned_connection_state = connection_state.clone();
            tasks.push((
                connection_id,
                tokio::spawn(
                    async move { cloned_connection_state.send_message(cloned_message).await },
                ),
            ));
        }

        // collect send results
        let mut has_errored = false;
        let mut results = Vec::new();
        for (connection_id, task) in tasks {
            let result = match task.await {
                Ok(Err(hyper_tungstenite::tungstenite::Error::ConnectionClosed)) => continue, // connection closed -> skip
                Ok(Err(hyper_tungstenite::tungstenite::Error::AlreadyClosed)) => {
                    tracing::debug!("connection {connection_id:?} already closed, please check if connections are deleted properly when closed");
                    continue;
                }
                Ok(Err(tungstenite_error)) => {
                    has_errored = true;
                    Err(SingleSendError::new(
                        tungstenite_error.into(),
                        connection_id,
                    ))
                }
                Err(join_error) => {
                    has_errored = true;
                    Err(SingleSendError::new(join_error.into(), connection_id))
                }
                Ok(Ok(())) => Ok(()),
            };

            results.push(result);
        }

        if has_errored {
            return Err(BatchSendError(results));
        }

        Ok(results.len())
    }
}

// TODO: Tests
