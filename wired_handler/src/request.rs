use std::{ops::ControlFlow, sync::Arc};

use futures_util::Future;
use tokio::sync::RwLock;

use crate::{GlobalState, SessionState, StateHolder};

/// Holds all relevant request data for handling it
#[derive(Debug)]
pub struct RequestCtx {
    state: StateHolder,
    session_ctx: Arc<RwLock<SessionState>>,
    global_ctx: Arc<RwLock<GlobalState>>,
}

impl RequestCtx {
    pub(crate) fn new(
        session_ctx: Arc<RwLock<SessionState>>,
        global_ctx: Arc<RwLock<GlobalState>>,
    ) -> Self {
        Self {
            state: StateHolder::default(),
            session_ctx,
            global_ctx,
        }
    }
}

// GET
impl RequestCtx {
    /// Gets request data by type as clone
    pub fn get_request_cloned<T: Clone + Send + Sync + 'static>(&self) -> Option<T> {
        self.state.get::<T>().cloned()
    }

    /// Gets request data by type as reference
    pub fn get_request<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.state.get::<T>()
    }

    /// Gets session data by type as clone
    pub async fn get_session<T: Clone + Send + Sync + 'static>(&self) -> Option<T> {
        let session_ctx = self.session_ctx.read().await;
        session_ctx.get::<T>().cloned()
    }

    /// Gets global data by type as clone
    pub async fn get_global<T: Clone + Send + Sync + 'static>(&self) -> Option<T> {
        let global_ctx = self.global_ctx.read().await;
        global_ctx.get::<T>().cloned()
    }
}

// PROVIDE
impl RequestCtx {
    /// Inserts request data by type, replaced if already present
    pub fn provide_request<T: Send + Sync + 'static>(&mut self, data: T) {
        self.state.provide(data);
    }

    /// Inserts session data by type, replaced if already present
    pub async fn provide_session<T: Send + Sync + 'static>(&self, data: T) {
        let mut session_ctx = self.session_ctx.write().await;

        session_ctx.provide(data);
    }

    /// Inserts global data by type, replaced if already present
    pub async fn provide_global<T: Send + Sync + 'static>(&self, data: T) {
        let mut global_ctx = self.global_ctx.write().await;

        global_ctx.provide(data);
    }
}

// REMOVE
impl RequestCtx {
    /// Removes request data by type and returns it
    pub fn remove_request<T: Send + Sync + 'static>(&mut self) -> Option<T> {
        self.state.remove::<T>()
    }

    /// Removes session data by type and returns it
    pub async fn remove_session<T: Send + Sync + 'static>(&self) -> Option<T> {
        let mut session_ctx = self.session_ctx.write().await;

        session_ctx.state.remove::<T>()
    }

    /// Removes global data by type and returns it
    pub async fn remove_global<T: Send + Sync + 'static>(&self) -> Option<T> {
        let mut global_ctx = self.global_ctx.write().await;

        global_ctx.state.remove::<T>()
    }
}

// UPDATE SYNC
impl RequestCtx {
    /// Takes a callback to update request data by type
    pub fn update_request<T: Send + Sync + 'static, E>(
        &mut self,
        handler: impl FnOnce(Option<T>) -> Result<Option<T>, E>,
    ) {
        let old_value = self.state.remove::<T>();

        if let Ok(Some(new_value)) = handler(old_value) {
            self.state.provide::<T>(new_value)
        }
    }

    /// Takes a callback to update session data by type
    pub async fn update_session<T: Send + Sync + 'static>(
        &self,
        handler: impl FnOnce(Option<T>) -> Option<T>,
    ) {
        let mut session_ctx = self.session_ctx.write().await;

        let old_value = session_ctx.state.remove::<T>();

        if let Some(new_value) = handler(old_value) {
            session_ctx.provide::<T>(new_value)
        }
    }

    /// Takes a callback to update global data by type
    pub async fn update_global<T: Send + Sync + 'static>(
        &self,
        handler: impl FnOnce(Option<T>) -> Option<T>,
    ) {
        let mut global_ctx = self.global_ctx.write().await;

        let old_value = global_ctx.state.remove::<T>();

        if let Some(new_value) = handler(old_value) {
            global_ctx.provide::<T>(new_value)
        }
    }
}

// UPDATE FALLIBLE
impl RequestCtx {
    /// Takes a callback to update request data by type, can return an error
    pub fn update_request_fallible<T: Send + Sync + 'static, E>(
        &mut self,
        handler: impl FnOnce(Option<T>) -> Result<Option<T>, E>,
    ) -> Result<(), E> {
        let old_value = self.state.remove::<T>();

        if let Some(new_value) = handler(old_value)? {
            self.state.provide::<T>(new_value);
        }

        Ok(())
    }

    /// Takes a callback to update session data by type, can return an error
    pub async fn update_session_fallible<T: Send + Sync + 'static, E>(
        &self,
        handler: impl FnOnce(Option<T>) -> Result<Option<T>, E>,
    ) -> Result<(), E> {
        let mut session_ctx = self.session_ctx.write().await;

        let old_value = session_ctx.state.remove::<T>();

        if let Some(new_value) = handler(old_value)? {
            session_ctx.provide::<T>(new_value);
        }

        Ok(())
    }

    /// Takes a callback to update global data by type, can return an error
    pub async fn update_global_fallible<T: Send + Sync + 'static, E>(
        &self,
        handler: impl FnOnce(Option<T>) -> Result<Option<T>, E>,
    ) -> Result<(), E> {
        let mut global_ctx = self.global_ctx.write().await;

        let old_value = global_ctx.state.remove::<T>();

        if let Some(new_value) = handler(old_value)? {
            global_ctx.provide::<T>(new_value);
        }

        Ok(())
    }
}

// UPDATE ASYNC
impl RequestCtx {
    /// Takes an async callback to update request data by type
    pub async fn update_request_async<T: Send + Sync + 'static, F: Future<Output = Option<T>>>(
        &mut self,
        handler: impl FnOnce(Option<T>) -> F,
    ) {
        let old_value = self.state.remove::<T>();
        if let Some(new_value) = handler(old_value).await {
            self.state.provide::<T>(new_value)
        }
    }

    /// Takes an async callback to update session data by type
    pub async fn update_session_async<T: Send + Sync + 'static, F: Future<Output = Option<T>>>(
        &self,
        handler: impl FnOnce(Option<T>) -> F,
    ) {
        let mut session_ctx = self.session_ctx.write().await;

        let old_value = session_ctx.state.remove::<T>();

        if let Some(new_value) = handler(old_value).await {
            session_ctx.provide::<T>(new_value)
        }
    }

    /// Takes an async callback to update global data by type
    pub async fn update_global_async<T: Send + Sync + 'static, F: Future<Output = Option<T>>>(
        &self,
        handler: impl FnOnce(Option<T>) -> F,
    ) {
        let mut global_ctx = self.global_ctx.write().await;

        let old_value = global_ctx.state.remove::<T>();

        if let Some(new_value) = handler(old_value).await {
            global_ctx.provide::<T>(new_value)
        }
    }
}

// UPDATE ASYNC FALLIBLE
impl RequestCtx {
    /// Takes an async callback to update request data by type, can return an error
    pub async fn update_request_async_fallible<
        T: Send + Sync + 'static,
        F: Future<Output = Result<Option<T>, E>>,
        E,
    >(
        &mut self,
        handler: impl FnOnce(Option<T>) -> F,
    ) -> Result<(), E> {
        let old_value = self.state.remove::<T>();

        if let Some(new_value) = handler(old_value).await? {
            self.state.provide::<T>(new_value);
        }

        Ok(())
    }

    /// Takes a callback to update session data by type, can return an error
    pub async fn update_session_async_fallible<
        T: Send + Sync + 'static,
        F: Future<Output = Result<Option<T>, E>>,
        E,
    >(
        &self,
        handler: impl FnOnce(Option<T>) -> F,
    ) -> Result<(), E> {
        let mut session_ctx = self.session_ctx.write().await;

        let old_value = session_ctx.state.remove::<T>();

        if let Some(new_value) = handler(old_value).await? {
            session_ctx.provide::<T>(new_value);
        }

        Ok(())
    }

    /// Takes a callback to update global data by type, can return an error
    pub async fn update_global_async_fallible<
        T: Send + Sync + 'static,
        F: Future<Output = Result<Option<T>, E>>,
        E,
    >(
        &self,
        handler: impl FnOnce(Option<T>) -> F,
    ) -> Result<(), E> {
        let mut global_ctx = self.global_ctx.write().await;

        let old_value = global_ctx.state.remove::<T>();

        if let Some(new_value) = handler(old_value).await? {
            global_ctx.provide::<T>(new_value);
        }

        Ok(())
    }
}

/// Can be handled by a `Router` when implemented
pub trait Request {
    type Error: Send + Sync + 'static;
    /// Applies the request data to the `RequestCtx` by providing needed data
    async fn apply_ctx(self, ctx: &mut RequestCtx) -> Result<ControlFlow<()>, Self::Error>;
}
