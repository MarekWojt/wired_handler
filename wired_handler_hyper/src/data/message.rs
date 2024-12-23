use hyper_tungstenite::tungstenite::Message;
use wired_handler::GetState;

use crate::{
    prelude::*,
    state::{context::WebsocketRequestContext, request_state::RequestState},
};

/// For getting the Message from a websocket context
pub trait ContextMessageExt {
    fn message(&self) -> &Message;
    fn message_mut(&mut self) -> &mut Message;
}

impl ContextMessageExt for WebsocketRequestContext {
    fn message(&self) -> &Message {
        RequestState::get_from_ctx(self)
            .get::<Message>()
            .expect("WebsocketRequestContext must have Message")
    }

    fn message_mut(&mut self) -> &mut Message {
        RequestState::get_mut_from_ctx(self)
            .get_mut::<Message>()
            .expect("WebsocketRequestContext must have Message")
    }
}
