use wired_handler::{Context, ContextBuilder, GetState};

use crate::state::{
    global_state::GlobalState, request_state::RequestState, session_state::SessionState,
};

/// The main context for http requests
#[derive(Debug, Context, ContextBuilder, GetState)]
#[builder_ident = "HttpRequestContextBuilder"]
pub struct HttpRequestContext {
    #[global_state]
    global_state: GlobalState,
    #[state]
    session_state: SessionState,
    #[state]
    request_state: RequestState,
}

impl HttpRequestContext {
    pub fn from_states(
        global_state: GlobalState,
        session_state: SessionState,
        request_state: RequestState,
    ) -> Self {
        Self {
            global_state,
            session_state,
            request_state,
        }
    }
}
