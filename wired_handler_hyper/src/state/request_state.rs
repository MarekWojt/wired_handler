use wired_handler::{
    plain::PlainState, State, StateSyncGet, StateSyncGetCloned, StateSyncMutableGetMut,
    StateSyncMutableGetMutOrInsert, StateSyncMutableInsert, StateSyncMutableRemoveGet,
};

/// Data specific to a request
#[derive(
    Debug,
    Default,
    State,
    StateSyncGet,
    StateSyncGetCloned,
    StateSyncMutableGetMut,
    StateSyncMutableInsert,
    StateSyncMutableRemoveGet,
    StateSyncMutableGetMutOrInsert,
)]
pub struct RequestState(PlainState);
