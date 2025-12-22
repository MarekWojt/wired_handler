use std::future::Future;

use futures::SinkExt;
use hyper_tungstenite::tungstenite::Message;
use tokio::time::timeout;

use super::GetMessageSinkExt;
use crate::{
    data::send_message::{SendMessageError, global_send_timeout},
    state::connection_state::ConnectionState,
};

/// For sending messages to a connection
pub trait SendMessageExt: GetMessageSinkExt {
    /// Sends a single Message
    fn send_message(&self, message: Message) -> impl Future<Output = Result<(), SendMessageError>> {
        async {
            let mut sink = self.message_sink_mut().await;
            match timeout(global_send_timeout(), sink.send(message)).await {
                Ok(Ok(())) => Ok(()),
                Ok(Err(tungstenite_error)) => Err(tungstenite_error.into()),
                Err(elapsed) => Err(elapsed.into()),
            }
        }
    }
}

impl SendMessageExt for ConnectionState {}
