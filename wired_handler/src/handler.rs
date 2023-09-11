use futures_util::{future::BoxFuture, Future};
use std::ops::ControlFlow;

use crate::RequestCtx;

/// A handler that can be used in the `Router`
pub type Handler<E> = fn(&mut RequestCtx) -> BoxFuture<'_, Result<ControlFlow<()>, E>>;

/// Turns an async function into a `Handler` (by boxing it)
#[macro_export]
macro_rules! make_handler {
    ($fn:expr) => {
        |ctx| ::std::boxed::Box::pin($crate::run_handler_and_convert_error($fn, ctx))
    };
}

/// Creates a list of `Handler`s
#[macro_export]
macro_rules! handlers {
    ([
        $($fn:expr),*
    ]) => {
        ::std::sync::Arc::new([
            $($crate::make_handler!($fn)),*
        ])
    };
}

/// Runs handler and converts its error if returned. Do not call this, this is
/// used by the `handlers!` and `make_handler` macro
pub async fn run_handler_and_convert_error<
    'a,
    EIn,
    EOut: From<EIn>,
    R: Future<Output = Result<ControlFlow<()>, EIn>>,
    F: FnOnce(&'a mut RequestCtx) -> R,
>(
    callback: F,
    ctx: &'a mut RequestCtx,
) -> Result<ControlFlow<()>, EOut> {
    Ok(callback(ctx).await?)
}
