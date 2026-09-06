use std::ops::ControlFlow;

use hyper::StatusCode;

use crate::{
    data::response_body::ResponseBody,
    prelude::*,
    state::{
        context::HttpRequestContext, global_state::GlobalState, request_state::RequestState,
        session_state::SessionState,
    },
};

use super::Response;

fn empty_context() -> HttpRequestContext {
    HttpRequestContext::from_states(
        GlobalState::default(),
        SessionState::default(),
        RequestState::default(),
    )
}

fn response_with_status(status: StatusCode) -> Response {
    Response::builder()
        .status(status)
        .body(ResponseBody::empty())
        .unwrap()
}

#[test]
fn test() {
    // next
    {
        let mut ctx = empty_context();

        assert!(matches!(
            ctx.next(response_with_status(StatusCode::OK)),
            Ok(ControlFlow::Continue(()))
        ));

        let stored = RequestState::get_from_ctx(&ctx).get::<Response>().unwrap();
        assert_eq!(stored.status(), StatusCode::OK);
    }

    // stop
    {
        let mut ctx = empty_context();

        assert!(matches!(
            ctx.stop(response_with_status(StatusCode::CREATED)),
            Ok(ControlFlow::Break(()))
        ));

        let stored = RequestState::get_from_ctx(&ctx).get::<Response>().unwrap();
        assert_eq!(stored.status(), StatusCode::CREATED);
    }

    // next and stop override an already saved Response
    {
        let mut ctx = empty_context();

        assert!(matches!(
            ctx.next(response_with_status(StatusCode::OK)),
            Ok(ControlFlow::Continue(()))
        ));
        assert!(matches!(
            ctx.stop(response_with_status(StatusCode::NO_CONTENT)),
            Ok(ControlFlow::Break(()))
        ));

        let stored = RequestState::get_from_ctx(&ctx).get::<Response>().unwrap();
        assert_eq!(stored.status(), StatusCode::NO_CONTENT);
    }

    // next_no_response / stop_no_response don't touch the saved Response
    {
        let ctx = empty_context();

        assert!(matches!(
            ctx.next_no_response(),
            Ok(ControlFlow::Continue(()))
        ));
        assert!(matches!(ctx.stop_no_response(), Ok(ControlFlow::Break(()))));

        assert!(!RequestState::get_from_ctx(&ctx).exists::<Response>());
    }
}
