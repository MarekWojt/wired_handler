use wired_handler::{
    async_double_rwlock::AsyncDoubleRwLockState, State, StateAsyncGet, StateAsyncGetCloned,
    StateAsyncGetMut, StateAsyncGetMutOrInsert, StateAsyncInsert, StateAsyncRemoveGetCloned,
};

/// Persistent over requests
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
pub struct SessionState(AsyncDoubleRwLockState);
