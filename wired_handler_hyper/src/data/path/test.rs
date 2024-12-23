use wired_handler::StateSyncMutableInsert;

use crate::{
    prelude::*,
    state::{
        context::HttpRequestContext, global_state::GlobalState, request_state::RequestState,
        session_state::SessionState,
    },
};

fn context_with_path(path: &str) -> HttpRequestContext {
    let mut request_state = RequestState::default();
    request_state.insert(path.to_string());
    HttpRequestContext::from_states(
        GlobalState::default(),
        SessionState::default(),
        request_state,
    )
}

#[test]
fn test() {
    // forward
    {
        let mut ctx = context_with_path("");
        let remaining_path = ctx.remaining_path_mut();
        assert_eq!(Some(""), remaining_path.peek().as_deref());
        assert_eq!(Some(""), remaining_path.next().as_deref());
        assert_eq!(None, remaining_path.peek().as_deref());
        assert_eq!(None, remaining_path.next().as_deref());
        assert_eq!(None, remaining_path.peek().as_deref());
        assert_eq!(None, remaining_path.next().as_deref());
    }

    {
        let mut ctx = context_with_path("/");
        let remaining_path = ctx.remaining_path_mut();
        assert_eq!(Some(""), remaining_path.peek().as_deref());
        assert_eq!(Some(""), remaining_path.next().as_deref());
        assert_eq!(None, remaining_path.peek().as_deref());
        assert_eq!(None, remaining_path.next().as_deref());
        assert_eq!(None, remaining_path.peek().as_deref());
        assert_eq!(None, remaining_path.next().as_deref());
    }

    {
        let mut ctx = context_with_path("/test");
        let remaining_path = ctx.remaining_path_mut();
        assert_eq!(Some("test"), remaining_path.peek().as_deref());
        assert_eq!(Some("test"), remaining_path.next().as_deref());
        assert_eq!(None, remaining_path.peek().as_deref());
        assert_eq!(None, remaining_path.next().as_deref());
        assert_eq!(None, remaining_path.peek().as_deref());
        assert_eq!(None, remaining_path.next().as_deref());
    }

    {
        let mut ctx = context_with_path("/test/bla");
        let remaining_path = ctx.remaining_path_mut();
        assert_eq!(Some("test"), remaining_path.peek().as_deref());
        assert_eq!(Some("test"), remaining_path.next().as_deref());
        assert_eq!(Some("bla"), remaining_path.peek().as_deref());
        assert_eq!(Some("bla"), remaining_path.next().as_deref());
        assert_eq!(None, remaining_path.peek().as_deref());
        assert_eq!(None, remaining_path.next().as_deref());
        assert_eq!(None, remaining_path.peek().as_deref());
        assert_eq!(None, remaining_path.next().as_deref());
    }

    {
        let mut ctx = context_with_path("/test/first/second/last");
        let remaining_path = ctx.remaining_path_mut();
        assert_eq!(Some("test"), remaining_path.peek().as_deref());
        assert_eq!(Some("test"), remaining_path.next().as_deref());
        assert_eq!(Some("first"), remaining_path.peek().as_deref());
        assert_eq!(Some("first"), remaining_path.next().as_deref());
        assert_eq!(Some("second"), remaining_path.peek().as_deref());
        assert_eq!(Some("second"), remaining_path.next().as_deref());
        assert_eq!(Some("last"), remaining_path.peek().as_deref());
        assert_eq!(Some("last"), remaining_path.next().as_deref());
        assert_eq!(None, remaining_path.peek().as_deref());
        assert_eq!(None, remaining_path.next().as_deref());
        assert_eq!(None, remaining_path.peek().as_deref());
        assert_eq!(None, remaining_path.next().as_deref());
    }

    // backwards
    {
        let mut ctx = context_with_path("");
        let remaining_path = ctx.remaining_path_mut();
        assert_eq!(Some(""), remaining_path.peek_back().as_deref());
        assert_eq!(Some(""), remaining_path.next_back().as_deref());
        assert_eq!(None, remaining_path.peek_back().as_deref());
        assert_eq!(None, remaining_path.next_back().as_deref());
        assert_eq!(None, remaining_path.peek_back().as_deref());
        assert_eq!(None, remaining_path.next_back().as_deref());
    }

    {
        let mut ctx = context_with_path("/");
        let remaining_path = ctx.remaining_path_mut();
        assert_eq!(Some(""), remaining_path.peek_back().as_deref());
        assert_eq!(Some(""), remaining_path.next_back().as_deref());
        assert_eq!(None, remaining_path.peek_back().as_deref());
        assert_eq!(None, remaining_path.next_back().as_deref());
        assert_eq!(None, remaining_path.peek_back().as_deref());
        assert_eq!(None, remaining_path.next_back().as_deref());
    }

    {
        let mut ctx = context_with_path("/test");
        let remaining_path = ctx.remaining_path_mut();
        assert_eq!(Some("test"), remaining_path.peek_back().as_deref());
        assert_eq!(Some("test"), remaining_path.next_back().as_deref());
        assert_eq!(None, remaining_path.peek_back().as_deref());
        assert_eq!(None, remaining_path.next_back().as_deref());
        assert_eq!(None, remaining_path.peek_back().as_deref());
        assert_eq!(None, remaining_path.next_back().as_deref());
    }

    {
        let mut ctx = context_with_path("/test/bla");
        let remaining_path = ctx.remaining_path_mut();
        assert_eq!(Some("bla"), remaining_path.peek_back().as_deref());
        assert_eq!(Some("bla"), remaining_path.next_back().as_deref());
        assert_eq!(Some("test"), remaining_path.peek_back().as_deref());
        assert_eq!(Some("test"), remaining_path.next_back().as_deref());
        assert_eq!(None, remaining_path.peek_back().as_deref());
        assert_eq!(None, remaining_path.next_back().as_deref());
        assert_eq!(None, remaining_path.peek_back().as_deref());
        assert_eq!(None, remaining_path.next_back().as_deref());
    }

    {
        let mut ctx = context_with_path("/test/first/second/last");
        let remaining_path = ctx.remaining_path_mut();
        assert_eq!(Some("last"), remaining_path.peek_back().as_deref());
        assert_eq!(Some("last"), remaining_path.next_back().as_deref());
        assert_eq!(Some("second"), remaining_path.peek_back().as_deref());
        assert_eq!(Some("second"), remaining_path.next_back().as_deref());
        assert_eq!(Some("first"), remaining_path.peek_back().as_deref());
        assert_eq!(Some("first"), remaining_path.next_back().as_deref());
        assert_eq!(Some("test"), remaining_path.peek_back().as_deref());
        assert_eq!(Some("test"), remaining_path.next_back().as_deref());
        assert_eq!(None, remaining_path.peek_back().as_deref());
        assert_eq!(None, remaining_path.next_back().as_deref());
        assert_eq!(None, remaining_path.peek_back().as_deref());
        assert_eq!(None, remaining_path.next_back().as_deref());
    }
}
