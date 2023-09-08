use futures_util::future::BoxFuture;
use std::ops::ControlFlow;

use crate::RequestCtx;

pub type Handler<E> = fn(&mut RequestCtx) -> BoxFuture<'_, Result<ControlFlow<()>, E>>;

/// Turns an async function into a `Handler` (by boxing it)
#[macro_export]
macro_rules! make_handler {
    ($fn:expr) => {
        |ctx| ::std::boxed::Box::pin($fn(ctx))
    };
}

/// Convenience macro for creating the list of `Handler`s
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
