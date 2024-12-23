use std::{
    future::Future,
    ops::{Deref, DerefMut},
};

use futures::stream::SplitSink;
use hyper::upgrade::Upgraded;
use hyper_tungstenite::{tungstenite::Message, WebSocketStream};
use hyper_util::rt::TokioIo;

use crate::{prelude::*, state::connection_state::ConnectionState};

pub type MessageSink = SplitSink<WebSocketStream<TokioIo<Upgraded>>, Message>;

/// For getting the message sink of a connection
pub trait GetMessageSinkExt {
    fn message_sink(&self) -> impl Future<Output = impl Deref<Target = MessageSink>>;
    fn message_sink_mut(&self) -> impl Future<Output = impl DerefMut<Target = MessageSink>>;
}

impl GetMessageSinkExt for ConnectionState {
    async fn message_sink(&self) -> impl Deref<Target = MessageSink> {
        self.get().await.expect("ConnectionState must have Sink")
    }

    async fn message_sink_mut(&self) -> impl DerefMut<Target = MessageSink> {
        self.get_mut()
            .await
            .expect("ConnectionState must have Sink")
    }
}
