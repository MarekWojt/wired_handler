use wired_handler::{Context, ContextBuilder, GetState};

use crate::state::{global_state::GlobalState, request_state::RequestState};

/// To be converted to `RequestContext` by assigning `SessionState`
#[derive(Debug, Context, ContextBuilder, GetState)]
#[builder_ident = "SessionlessRequestContextBuilder"]
pub struct SessionlessRequestContext {
    #[global_state]
    global_state: GlobalState,
    #[state]
    request_state: RequestState,
}

impl SessionlessRequestContext {
    pub fn into_states(self) -> (GlobalState, RequestState) {
        (self.global_state, self.request_state)
    }
}
