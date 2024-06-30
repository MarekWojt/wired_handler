use std::ops::{Deref, DerefMut};

use crate::state::State;

/// Marks an objekt as context
pub trait Context {}

/// For extracting the `State` `Self` from a `Context`
pub trait GetState<C: Context>: State {
    /// Immutably gets the `State` `Self` from the `Context` `C`
    fn get_from_ctx(ctx: &C)
        -> impl std::future::Future<Output = impl Deref<Target = Self>> + Send;
    /// Mutably gets the `State` `Self` from the `Context` `C`
    fn get_mut_from_ctx(
        ctx: &mut C,
    ) -> impl std::future::Future<Output = impl DerefMut<Target = Self>> + Send;
}

/// For building a context, given all fields except for the global state `S`
pub trait ContextBuilder<S> {
    /// The error returned when building the `Context` fails
    type Error;
    /// The `Context` built
    type Output: Context;

    /// Builds the `Context` `Self::Output` with the global state `S`
    fn build(
        self,
        global_state: S,
    ) -> impl std::future::Future<Output = Result<Self::Output, Self::Error>> + Send;
}
