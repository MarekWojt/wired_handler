use crate::state::State;

/// Marks an object as context
pub trait Context {}

/// For extracting the `State` `Self` from a `Context`
pub trait GetState<C: Context>: State {
    /// Immutably gets the `State` `Self` from the `Context` `C`
    fn get_from_ctx(ctx: &C) -> &Self;
    /// Mutably gets the `State` `Self` from the `Context` `C`
    fn get_mut_from_ctx(ctx: &mut C) -> &mut Self;
}

/// For building a context, given all fields except for the global state `S`
pub trait ContextBuilder<S> {
    /// The `Context` built
    type Output: Context;

    /// Builds the `Context` `Self::Output` with the global state `S`
    fn build(self, global_state: S) -> impl std::future::Future<Output = Self::Output> + Send;
}
