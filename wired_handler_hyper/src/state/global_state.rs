use wired_handler::{
    async_double_rwlock::AsyncDoubleRwLockState, State, StateAsyncGet, StateAsyncGetCloned,
    StateAsyncGetMut, StateAsyncGetMutOrInsert, StateAsyncInsert, StateAsyncRemoveGetCloned,
};

/// Should only exist once
#[derive(
    Debug,
    Default,
    Clone,
    State,
    StateAsyncGet,
    StateAsyncGetCloned,
    StateAsyncGetMut,
    StateAsyncInsert,
    StateAsyncRemoveGetCloned,
    StateAsyncGetMutOrInsert,
)]
pub struct GlobalState(AsyncDoubleRwLockState);
