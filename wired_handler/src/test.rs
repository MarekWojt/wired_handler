#![cfg(test)]

use std::{ops::ControlFlow, sync::Arc};

use tokio::{runtime::Runtime, sync::RwLock};

use crate::{handlers, GlobalState, Request, RequestCtx, Router, SessionState};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Counter(i32);

#[derive(Debug, Clone, PartialEq, Eq)]
struct ThirdHandlerExecuted;

async fn first_handler(ctx: &mut RequestCtx) -> Result<ControlFlow<()>, String> {
    ctx.update_global::<Counter>(|old_value| {
        let mut counter = old_value.unwrap_or(Counter(0));

        counter.0 += 1;

        Some(counter)
    })
    .await;

    Ok(ControlFlow::Continue(()))
}

async fn second_handler(ctx: &mut RequestCtx) -> Result<ControlFlow<()>, String> {
    if ctx.get_request::<i32>() == Some(&12) {
        return Err("Fehler".to_string());
    }

    if ctx.get_request::<i32>() == Some(&42) {
        return Ok(ControlFlow::Break(()));
    }

    Ok(ControlFlow::Continue(()))
}

async fn third_handler(ctx: &mut RequestCtx) -> Result<ControlFlow<()>, String> {
    ctx.provide_request(ThirdHandlerExecuted);

    Ok(ControlFlow::Continue(()))
}

#[derive(Debug, PartialEq, Eq)]
struct Error(String);

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self(value)
    }
}

struct SomeRequest(i32);

impl Request for SomeRequest {
    type Error = String;

    async fn apply_ctx(self, ctx: &mut RequestCtx) -> Result<ControlFlow<()>, Self::Error> {
        ctx.provide_request(self.0);

        ctx.update_session(|old_state| Some(old_state.unwrap_or(0) + self.0))
            .await;

        Ok(ControlFlow::Continue(()))
    }
}

#[test]
fn run_test() {
    let runtime = Runtime::new().unwrap();

    runtime.block_on(test());
}

async fn test() {
    let state = GlobalState::default();
    let handler = Router::<Error>::new(
        handlers!([first_handler, second_handler, third_handler]),
        Arc::new(RwLock::new(state)),
    );

    let session1: Arc<RwLock<SessionState>> = Arc::new(RwLock::new(SessionState::default()));
    let session2 = Arc::new(RwLock::new(SessionState::default()));

    {
        let result = handler.handle(SomeRequest(1), session1.clone()).await;
        assert_eq!(result.get_request::<i32>(), Some(&1));
        assert_eq!(result.get_session::<i32>().await, Some(1));
        assert_eq!(result.get_global::<Counter>().await, Some(Counter(1)));
        assert_eq!(result.get_request::<Error>(), None);
        assert_eq!(
            result.get_request::<ThirdHandlerExecuted>(),
            Some(&ThirdHandlerExecuted)
        );
    }

    {
        let result = handler.handle(SomeRequest(2), session1.clone()).await;
        assert_eq!(result.get_request::<i32>(), Some(&2));
        assert_eq!(result.get_session::<i32>().await, Some(3)); // 1 + 2 = 3
        assert_eq!(result.get_global::<Counter>().await, Some(Counter(2)));
        assert_eq!(result.get_request::<Error>(), None);
        assert_eq!(
            result.get_request::<ThirdHandlerExecuted>(),
            Some(&ThirdHandlerExecuted)
        );
    }

    {
        let result = handler.handle(SomeRequest(2), session2).await;
        assert_eq!(result.get_request::<i32>(), Some(&2));
        assert_eq!(result.get_session::<i32>().await, Some(2));
        assert_eq!(result.get_global::<Counter>().await, Some(Counter(3)));
        assert_eq!(result.get_request::<Error>(), None);
        assert_eq!(
            result.get_request::<ThirdHandlerExecuted>(),
            Some(&ThirdHandlerExecuted)
        );
    }

    {
        let result = handler.handle(SomeRequest(12), session1.clone()).await;
        assert_eq!(result.get_request::<i32>(), Some(&12));
        assert_eq!(result.get_session::<i32>().await, Some(15)); // 1 + 2 + 12 = 15
        assert_eq!(result.get_global::<Counter>().await, Some(Counter(4)));
        assert_eq!(
            result.get_request::<Error>(),
            Some(&Error("Fehler".to_string()))
        );
        assert_eq!(result.get_request::<ThirdHandlerExecuted>(), None);
    }

    {
        let result = handler.handle(SomeRequest(42), session1.clone()).await;
        assert_eq!(result.get_request::<i32>(), Some(&42));
        assert_eq!(result.get_session::<i32>().await, Some(57)); // 1 + 2 + 12 + 42 = 57
        assert_eq!(result.get_global::<Counter>().await, Some(Counter(5)));
        assert_eq!(result.get_request::<String>(), None);
        assert_eq!(result.get_request::<ThirdHandlerExecuted>(), None);
    }
}
