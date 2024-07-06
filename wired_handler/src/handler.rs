use std::future::Future;

use crate::{Context, ContextBuilder, State};

/// For handling requests, holds the global `State`
#[derive(Debug)]
pub struct Handler<CIn: Context, COut: Context, S: State + Clone, F: Future<Output = COut>> {
    state: S,
    handler: fn(CIn) -> F,
}

impl<CIn: Context, COut: Context, S: State + Clone, F: Future<Output = COut>> Clone
    for Handler<CIn, COut, S, F>
{
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            handler: self.handler,
        }
    }
}

impl<CIn: Context, COut: Context, S: State + Clone, F: Future<Output = COut>>
    Handler<CIn, COut, S, F>
{
    /// Creates a new `Handler` from the global `State` `S` and a handler fn
    pub fn new(state: S, handler: fn(CIn) -> F) -> Self {
        Self { state, handler }
    }

    /// Gets the global state
    pub fn state(&self) -> &S {
        &self.state
    }

    /// Gets the global state mutably
    pub fn state_mut(&mut self) -> &mut S {
        &mut self.state
    }

    /// Handles a request. Accepts a `ContextBuilder` `B` which will be completed to a `Context` with the global state `S`, which is then passed to the handler fn
    pub async fn handle<B: ContextBuilder<S, Output = CIn>>(&self, builder: B) -> COut {
        let ctx = builder.build(self.state.clone());
        let handler = self.handler;
        handler(ctx).await
    }
}
