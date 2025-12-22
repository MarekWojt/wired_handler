/// Runs a handler/middleware. Interrupts execution by `return`ing if return value is `Err` or `Break`
#[macro_export]
macro_rules! run_handler {
    ($handler:expr) => {
        if let ::std::ops::ControlFlow::Break(_) = $handler? {
            return ::std::result::Result::Ok(::std::ops::ControlFlow::Break(()));
        }
    };
}

/// Sugar for definining routes. Takes the next path element and matches it. Returns an error to the client for non-existing routes
#[macro_export]
macro_rules! routes {
    {
        $ctx:expr,
        $($path:pat => $handler:expr),*$(,)?
    } => {
        {
            use $crate::prelude::{ContextGetPathExt, ContextGetRequestExt};
            match $ctx.remaining_path_mut().next().as_deref() {
                $(
                    $path => $handler,
                )*
                _ => match $ctx.request().method() {
                    &::http::Method::GET | &::http::Method::HEAD => ::std::result::Result::Err($crate::data::http_error::HttpError::not_found("not found")),
                    _ => ::std::result::Result::Err($crate::data::http_error::HttpError::not_implemented("not implemented")),
                },
            }
        }
    };
}

/// Sugar for definining actions by method. Creates an OPTIONS endpoint as well, so use it as late as possible. Returns an error to the client for non-existing routes
#[macro_export]
macro_rules! actions {
    {
        $ctx:expr,
        $($methods:pat => $handler:expr),*$(,)?
    } => {
        {
            use $crate::prelude::{ContextGetRequestExt, ContextReturnResponseExt};
            match $ctx.request().method() {
                $(
                    &$methods => $handler,
                )*
                &::http::Method::OPTIONS => $ctx.next($crate::data::response::Response::builder().body($crate::data::response_body::ResponseBody::empty())?),

                #[allow(unreachable_patterns)]
                &::http::Method::GET | &::http::Method::HEAD => ::std::result::Result::Err($crate::data::http_error::HttpError::not_found("not found")),

                _ => ::std::result::Result::Err($crate::data::http_error::HttpError::not_implemented("not implemented")),
            }
        }
    };
}

/// test `run_handler!`
#[cfg(test)]
mod test_run_handler {
    #![allow(clippy::result_large_err)]
    use crate::data::http_error::HttpError;
    use std::ops::ControlFlow;

    fn test_break() -> Result<ControlFlow<()>, HttpError> {
        run_handler!(Result::<ControlFlow<()>, HttpError>::Ok(
            ControlFlow::Break(())
        ));
        Ok(ControlFlow::Continue(()))
    }

    fn test_err() -> Result<ControlFlow<()>, HttpError> {
        run_handler!(Result::<ControlFlow<()>, HttpError>::Err(
            HttpError::internal_server_error("test")
        ));
        Ok(ControlFlow::Continue(()))
    }

    fn test_continue() -> Result<ControlFlow<()>, HttpError> {
        run_handler!(Result::<ControlFlow<()>, HttpError>::Ok(
            ControlFlow::Continue(())
        ));
        Ok(ControlFlow::Break(()))
    }

    #[test]
    fn test() {
        assert!(test_break().unwrap().is_break());
        assert!(test_err().is_err());
        assert!(test_continue().unwrap().is_break());
    }
}
