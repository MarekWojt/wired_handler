use std::future::Future;

use futures::SinkExt;
use hyper_tungstenite::tungstenite::Message;

use super::GetMessageSinkExt;
use crate::state::connection_state::ConnectionState;

/// For sending messages to a connection
pub trait SendMessageExt: GetMessageSinkExt {
    /// Sends a single Message
    fn send_message(
        &self,
        message: Message,
    ) -> impl Future<Output = Result<(), hyper_tungstenite::tungstenite::Error>> {
        async {
            let mut sink = self.message_sink_mut().await;
            sink.send(message).await
        }
    }
}

impl SendMessageExt for ConnectionState {}
