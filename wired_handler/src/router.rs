use std::{fmt::Debug, ops::ControlFlow, sync::Arc};

use tokio::sync::RwLock;

use crate::{Handler, Request, RequestCtx, StateHolder};

/// Combines one or multiple `Handler`s and one `GlobalState` to handle requests
#[derive(Debug, Clone)]
pub struct Router<E: Send + Sync + 'static> {
    state: Arc<RwLock<GlobalState>>,
    handlers: Arc<[Handler<E>]>,
}

macro_rules! handle_handler {
    ($result:expr, $request_ctx:expr) => {{
        match $result {
            Err(err) => {
                $request_ctx.provide_request(err);
                return $request_ctx;
            }
            Ok(ControlFlow::Break(_)) => return $request_ctx,
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
    ) -> RequestCtx {
        let mut request_ctx = RequestCtx::new(session_state, self.state.clone());

        handle_handler!(request.apply_ctx(&mut request_ctx).await, request_ctx);

        for handler in self.handlers.iter() {
            handle_handler!(handler(&mut request_ctx).await, request_ctx);
        }

        request_ctx
    }
}

#[derive(Debug, Default)]
pub struct GlobalState {
    pub(crate) state: StateHolder,
}

macro_rules! state_wrapper {
    () => {
        pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
            self.state.get::<T>()
        }

        pub fn get_cloned<T: Clone + Send + Sync + 'static>(&self) -> Option<T> {
            self.get::<T>().map(|d| d.clone())
        }

        pub fn provide<T: Send + Sync + 'static>(&mut self, data: T) {
            self.state.provide(data)
        }

        pub fn remove<T: Send + Sync + 'static>(&mut self) {
            self.state.remove::<T>();
        }
    };
}

impl GlobalState {
    state_wrapper!();
}

#[derive(Debug, Default)]
pub struct SessionState {
    pub(crate) state: StateHolder,
}

impl SessionState {
    state_wrapper!();
}
