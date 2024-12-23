use wired_handler::{
    async_double_rwlock::AsyncDoubleRwLockState, State, StateAsyncGet, StateAsyncGetCloned,
    StateAsyncGetMut, StateAsyncGetMutOrInsert, StateAsyncInsert, StateAsyncRemoveGetCloned,
};

/// Holds data persistent over a connection
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
pub struct ConnectionState(AsyncDoubleRwLockState);
