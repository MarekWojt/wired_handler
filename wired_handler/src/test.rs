#![cfg(test)]
use std::collections::HashMap;

use crate::{
    plain::PlainState, Context, ContextBuilder, GetState, Handler, State, StateAsyncGet,
    StateAsyncGetCloned, StateAsyncGetMut, StateAsyncProvide, StateSyncGet, StateSyncGetCloned,
    StateSyncMutableGetMut, StateSyncMutableProvide,
};
use tokio::runtime::Runtime;

use crate::async_double_rwlock::AsyncDoubleRwLockState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SessionId(u32);

#[derive(Debug, Default)]
struct SessionStorage(HashMap<SessionId, SessionState>);

impl SessionStorage {
    fn add(&mut self, session_id: SessionId, session_state: SessionState) {
        self.0.insert(session_id, session_state);
    }

    fn get(&self, session_id: &SessionId) -> Option<&SessionState> {
        self.0.get(session_id)
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    State,
    StateAsyncGet,
    StateAsyncGetCloned,
    StateAsyncGetMut,
    StateAsyncProvide,
)]
struct GlobalState(AsyncDoubleRwLockState);

#[derive(
    Debug,
    Default,
    Clone,
    State,
    StateAsyncGet,
    StateAsyncGetCloned,
    StateAsyncGetMut,
    StateAsyncProvide,
)]
struct SessionState(AsyncDoubleRwLockState);

#[derive(
    Debug,
    Default,
    Clone,
    State,
    StateAsyncGet,
    StateAsyncGetCloned,
    StateAsyncGetMut,
    StateAsyncProvide,
)]
struct PreSessionState(AsyncDoubleRwLockState);

#[derive(
    Debug,
    Default,
    State,
    StateSyncGet,
    StateSyncGetCloned,
    StateSyncMutableGetMut,
    StateSyncMutableProvide,
)]
struct RequestState(PlainState);

#[derive(Debug, Default, Context, ContextBuilder, GetState)]
#[builder_ident = "StartContextBuilder"]
#[error_ident = "StartContextBuilderError"]
struct StartContext {
    #[global_state]
    global_state: GlobalState,
    #[state]
    request_state: RequestState,
    #[state]
    pre_session_state: PreSessionState,
}

impl StartContext {
    async fn get_or_create_session_state(&self, session_id: SessionId) -> SessionState {
        let global_state = GlobalState::get_from_ctx(self).await.clone();
        let session_state = {
            global_state
                .get::<SessionStorage>()
                .await
                .and_then(|storage| storage.get(&session_id).cloned())
        };
        if let Some(found_session_state) = session_state {
            return found_session_state;
        }
        let session_state = SessionState::default();
        if let Some(mut session_storage) = global_state.get_mut::<SessionStorage>().await {
            session_storage.add(session_id, session_state.clone());
        }
        session_state
    }
}

#[derive(Debug, Default, Context, ContextBuilder, GetState)]
#[builder_ident = "EndContextBuilder"]
#[error_ident = "EndContextBuilderError"]
struct EndContext {
    #[global_state]
    global_state: GlobalState,
    #[state]
    request_state: RequestState,
    #[state]
    session_state: SessionState,
}

impl From<StartContext> for EndContextBuilder {
    fn from(value: StartContext) -> Self {
        let mut builder = Self::new();
        builder.request_state(value.request_state);
        builder
    }
}

async fn start_handler(ctx: StartContext) -> EndContext {
    let session_state = {
        let pre_session_state = PreSessionState::get_from_ctx(&ctx).await;

        let session_id = pre_session_state
            .get_cloned()
            .await
            .expect("PreSessionState must have SessionId");

        ctx.get_or_create_session_state(session_id).await
    };

    let global_state = GlobalState::get_from_ctx(&ctx).await.clone();

    let mut builder = EndContextBuilder::from(ctx);
    builder.session_state(session_state);

    builder.build(global_state).await.unwrap()
}

async fn end_handler(ctx: &mut EndContext) {
    let session_state = SessionState::get_from_ctx(ctx).await;
    let request_state = RequestState::get_from_ctx(ctx).await;
    let increase_by = request_state.get_cloned::<u8>().unwrap_or(0);
    let mut current_value = match session_state.get_mut::<u8>().await {
        Some(value) => value,
        None => {
            session_state.provide(0u8).await;
            session_state.get_mut::<u8>().await.unwrap()
        }
    };
    *current_value += increase_by;
}

async fn handle(ctx: StartContext) -> EndContext {
    let mut end_context = start_handler(ctx).await;

    end_handler(&mut end_context).await;

    end_context
}

#[test]
fn run_test() {
    let runtime = Runtime::new().unwrap();

    runtime.block_on(test());
}

async fn test() {
    let state = GlobalState::default();
    state.provide(SessionStorage::default()).await;
    let handler = Handler::new(state, handle);
    let _ = handler.clone();

    let session1 = SessionId(0);
    let session2 = SessionId(1);
    {
        let mut ctx_builder = StartContextBuilder::new();
        let mut request_state = RequestState::default();
        request_state.provide(1u8);
        let pre_session_state = PreSessionState::default();
        pre_session_state.provide(session1).await;
        ctx_builder
            .request_state(request_state)
            .pre_session_state(pre_session_state);

        let end_ctx = handler.handle(ctx_builder).await.unwrap();
        let current_value = SessionState::get_from_ctx(&end_ctx)
            .await
            .get_cloned::<u8>()
            .await;

        assert_eq!(current_value, Some(1u8));
    }

    {
        let mut ctx_builder = StartContextBuilder::new();
        let mut request_state = RequestState::default();
        request_state.provide(2u8);
        let pre_session_state = PreSessionState::default();
        pre_session_state.provide(session1).await;
        ctx_builder
            .request_state(request_state)
            .pre_session_state(pre_session_state);

        let end_ctx = handler.handle(ctx_builder).await.unwrap();
        let current_value = SessionState::get_from_ctx(&end_ctx)
            .await
            .get_cloned::<u8>()
            .await;

        assert_eq!(current_value, Some(3u8));
    }

    {
        let mut ctx_builder = StartContextBuilder::new();
        let request_state = RequestState::default();
        let pre_session_state = PreSessionState::default();
        pre_session_state.provide(session2).await;
        ctx_builder
            .request_state(request_state)
            .pre_session_state(pre_session_state);

        let end_ctx = handler.handle(ctx_builder).await.unwrap();
        let current_value = SessionState::get_from_ctx(&end_ctx)
            .await
            .get_cloned::<u8>()
            .await;

        assert_eq!(current_value, Some(0u8));
    }

    {
        let mut ctx_builder = StartContextBuilder::new();
        let mut request_state = RequestState::default();
        request_state.provide(2u8);
        let pre_session_state = PreSessionState::default();
        pre_session_state.provide(session2).await;
        ctx_builder
            .request_state(request_state)
            .pre_session_state(pre_session_state);

        let end_ctx = handler.handle(ctx_builder).await.unwrap();
        let current_value = SessionState::get_from_ctx(&end_ctx)
            .await
            .get_cloned::<u8>()
            .await;

        assert_eq!(current_value, Some(2u8));
    }
}
