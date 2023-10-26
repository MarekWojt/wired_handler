use std::{fmt::Debug, ops::ControlFlow, sync::Arc};

use tokio::sync::RwLock;

use crate::{Handler, Request, RequestCtx, RequestResult, StateHolder};

/// Combines one or multiple `Handler`s and one `GlobalState` to handle requests
#[derive(Debug)]
pub struct Router<E: Send + Sync + 'static> {
    state: Arc<RwLock<GlobalState>>,
    handlers: Arc<[Handler<E>]>,
}

impl<E: Send + Sync + 'static> Clone for Router<E> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            handlers: self.handlers.clone(),
        }
    }
}

macro_rules! handle_handler {
    ($result:expr, $request_ctx:expr) => {{
        match $result {
            Err(err) => {
                return RequestResult($request_ctx, Err(err));
            }
            Ok(ControlFlow::Break(_)) => return RequestResult($request_ctx, Ok(())),
            _ => {}
        };
    }};
}

impl<E: Send + Sync + 'static> Router<E> {
    /// Creates a new `Router`. Use the `handlers!` macro to specify handlers.
    /// The returned error type does not have to be the exact type of `E`. `E`
    /// only needs to implement `From<YourErrorType>`
    pub fn new(handlers: Arc<[Handler<E>]>, state: Arc<RwLock<GlobalState>>) -> Self {
        Self { state, handlers }
    }

    /// Get the global state
    pub fn state(&self) -> &Arc<RwLock<GlobalState>> {
        &self.state
    }

    /// Handle a request
    pub async fn handle<R: Request>(
        &self,
        request: R,
        session_state: Arc<RwLock<SessionState>>,
    ) -> RequestResult<E>
    where
        E: From<R::Error>,
    {
        let mut request_ctx = RequestCtx::new(session_state, self.state.clone());

        handle_handler!(
            request
                .apply_ctx(&mut request_ctx)
                .await
                .map_err(|err| E::from(err)),
            request_ctx
        );

        for handler in self.handlers.iter() {
            handle_handler!(handler(&mut request_ctx).await, request_ctx);
        }

        RequestResult(request_ctx, Ok(()))
    }
}

/// Holds the global state
#[derive(Debug, Default)]
pub struct GlobalState {
    pub(crate) state: StateHolder,
}

macro_rules! state_wrapper {
    () => {
        /// Returns a reference to data by type
        pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
            self.state.get::<T>()
        }

        /// Returns data by type
        pub fn get_cloned<T: Clone + Send + Sync + 'static>(&self) -> Option<T> {
            self.get::<T>().map(|d| d.clone())
        }

        /// Inserts data by type
        pub fn provide<T: Send + Sync + 'static>(&mut self, data: T) {
            self.state.provide(data)
        }

        /// Removes data by type and returns it
        pub fn remove<T: Send + Sync + 'static>(&mut self) -> Option<T> {
            self.state.remove::<T>()
        }
    };
}

impl GlobalState {
    state_wrapper!();
}

/// Holds the session state
#[derive(Debug, Default)]
pub struct SessionState {
    pub(crate) state: StateHolder,
}

impl SessionState {
    state_wrapper!();
}
