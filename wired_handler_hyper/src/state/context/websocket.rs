use wired_handler::{Context, ContextBuilder, GetState};

use crate::state::{
    connection_state::ConnectionState, global_state::GlobalState, request_state::RequestState,
    session_state::SessionState,
};

/// The main context for websocket message handling
#[derive(Debug, Context, ContextBuilder, GetState)]
#[builder_ident = "WebsocketRequestContextBuilder"]
pub struct WebsocketRequestContext {
    #[global_state]
    global_state: GlobalState,
    #[state]
    session_state: SessionState,
    #[state]
    connection_state: ConnectionState,
    #[state]
    request_state: RequestState,
}

impl WebsocketRequestContext {
    pub fn from_states(
        global_state: GlobalState,
        session_state: SessionState,
        connection_state: ConnectionState,
        request_state: RequestState,
    ) -> Self {
        Self {
            global_state,
            session_state,
            connection_state,
            request_state,
        }
    }
}
