#![cfg(test)]
use std::collections::HashMap;

use crate::{
    plain::PlainState, Context, ContextBuilder, GetState, Handler, State, StateAsyncGet,
    StateAsyncGetCloned, StateAsyncGetMut, StateAsyncGetMutOrInsert, StateAsyncInsert,
    StateAsyncRemoveGetCloned, StateSyncGet, StateSyncGetCloned, StateSyncMutableGetMut,
    StateSyncMutableGetMutOrInsert, StateSyncMutableInsert, StateSyncMutableRemoveGet,
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
    StateAsyncInsert,
    StateAsyncGetMutOrInsert,
    StateAsyncRemoveGetCloned,
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
    StateAsyncInsert,
    StateAsyncGetMutOrInsert,
    StateAsyncRemoveGetCloned,
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
    StateAsyncInsert,
    StateAsyncGetMutOrInsert,
    StateAsyncRemoveGetCloned,
)]
struct PreSessionState(AsyncDoubleRwLockState);

#[derive(
    Debug,
    Default,
    State,
    StateSyncGet,
    StateSyncGetCloned,
    StateSyncMutableGetMut,
    StateSyncMutableInsert,
    StateSyncMutableGetMutOrInsert,
    StateSyncMutableRemoveGet,
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
        let global_state = GlobalState::get_from_ctx(self).clone();
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
        session_state.insert(session_id).await;
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
        let pre_session_state = PreSessionState::get_from_ctx(&ctx);

        let session_id = pre_session_state
            .get_cloned()
            .await
            .expect("PreSessionState must have SessionId");

        ctx.get_or_create_session_state(session_id).await
    };

    {
        let mut times_used = session_state.get_mut_or_insert_default::<u16>().await;
        *times_used += 1u16;
    }

    let global_state = GlobalState::get_from_ctx(&ctx).clone();

    let mut builder = EndContextBuilder::from(ctx);
    builder.session_state(session_state);

    builder.build(global_state).await.unwrap()
}

async fn end_handler(ctx: &mut EndContext) {
    {
        let session_state = SessionState::get_from_ctx(ctx);
        let request_state = RequestState::get_from_ctx(ctx);
        let increase_by = request_state.get_cloned::<u8>().unwrap_or(0);
        let mut current_value = session_state.get_mut_or_insert_default::<u8>().await;
        *current_value += increase_by;
    }
    let mut request_state_mut = RequestState::get_mut_from_ctx(ctx);
    if let Some(data) = request_state_mut.get_mut::<u8>() {
        *data *= 2;
    };
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
    state.insert(SessionStorage::default()).await;
    let handler = Handler::new(state, handle);
    let _ = handler.clone();

    let session1 = SessionId(0);
    let session2 = SessionId(1);
    {
        let mut ctx_builder = StartContextBuilder::new();
        let mut request_state = RequestState::default();
        request_state.insert(1u8);
        let pre_session_state = PreSessionState::default();
        pre_session_state.insert(session1).await;
        ctx_builder
            .request_state(request_state)
            .pre_session_state(pre_session_state);

        let mut end_ctx = handler.handle(ctx_builder).await.unwrap();
        {
            let session_state = SessionState::get_from_ctx(&end_ctx);
            let current_value = session_state.get_cloned::<u8>().await;
            assert_eq!(current_value, Some(1u8));

            let times_used = session_state.get_cloned::<u16>().await;
            assert_eq!(times_used, Some(1u16));
        }

        let request_value = RequestState::get_mut_from_ctx(&mut end_ctx).remove_get();
        assert_eq!(request_value, Some(2u8));
    }

    {
        let mut ctx_builder = StartContextBuilder::new();
        let mut request_state = RequestState::default();
        request_state.insert(2u8);
        let pre_session_state = PreSessionState::default();
        pre_session_state.insert(session1).await;
        ctx_builder
            .request_state(request_state)
            .pre_session_state(pre_session_state);

        let mut end_ctx = handler.handle(ctx_builder).await.unwrap();
        {
            let session_state = SessionState::get_from_ctx(&end_ctx);
            let current_value = session_state.get_cloned::<u8>().await;
            assert_eq!(current_value, Some(3u8));

            let times_used = session_state.get_cloned::<u16>().await;
            assert_eq!(times_used, Some(2u16));
        }

        let request_value = RequestState::get_mut_from_ctx(&mut end_ctx).remove_get();
        assert_eq!(request_value, Some(4u8));
    }

    {
        let mut ctx_builder = StartContextBuilder::new();
        let request_state = RequestState::default();
        let pre_session_state = PreSessionState::default();
        pre_session_state.insert(session2).await;
        ctx_builder
            .request_state(request_state)
            .pre_session_state(pre_session_state);

        let mut end_ctx = handler.handle(ctx_builder).await.unwrap();
        {
            let session_state = SessionState::get_from_ctx(&end_ctx);
            let current_value = session_state.get_cloned::<u8>().await;
            assert_eq!(current_value, Some(0u8));

            let times_used = session_state.get_cloned::<u16>().await;
            assert_eq!(times_used, Some(1u16));
        }

        let request_value: Option<u8> = RequestState::get_mut_from_ctx(&mut end_ctx).remove_get();
        assert_eq!(request_value, None);
    }

    {
        let mut ctx_builder = StartContextBuilder::new();
        let mut request_state = RequestState::default();
        request_state.insert(2u8);
        let pre_session_state = PreSessionState::default();
        pre_session_state.insert(session2).await;
        ctx_builder
            .request_state(request_state)
            .pre_session_state(pre_session_state);

        let mut end_ctx = handler.handle(ctx_builder).await.unwrap();
        {
            let session_state = SessionState::get_from_ctx(&end_ctx);
            let current_value = session_state.get_cloned::<u8>().await;
            assert_eq!(current_value, Some(2u8));

            let times_used = session_state.get_cloned::<u16>().await;
            assert_eq!(times_used, Some(2u16));
        }

        let request_value = RequestState::get_mut_from_ctx(&mut end_ctx).remove_get();
        assert_eq!(request_value, Some(4u8));
    }
}
