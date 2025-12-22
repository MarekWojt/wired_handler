use crate::{
    prelude::*,
    state::{context::HttpRequestContext, request_state::RequestState},
};

use super::RemainingPath;

/// For creating the `RemainingPath`
pub trait ContextCreateRemainingPathExt {
    /// Ensure a remaining path is in the state
    fn create_remaining_path(&mut self);
}

impl ContextCreateRemainingPathExt for HttpRequestContext {
    fn create_remaining_path(&mut self) {
        if RequestState::get_from_ctx(self).exists::<RemainingPath>() {
            return;
        }
        let path_without_starting_slash = {
            let path = self.path();
            let mut path_chars = path.chars();
            path_chars.next();

            path_chars.as_str().to_string()
        };
        RequestState::get_mut_from_ctx(self)
            .insert(RemainingPath(Some(path_without_starting_slash)));
    }
}

/// For retrieving the request path
pub trait ContextGetPathExt {
    /// Returns the full path of the request (starting at `"/"` and ending before `"&"`)
    fn path(&self) -> &str;
    /// Returns a reference to the `RemainingPath`. Created if it doesn't exist
    fn remaining_path(&mut self) -> &RemainingPath;
    /// Returns a mutable reference to the `RemainingPath`. Created if it doesn't exist
    fn remaining_path_mut(&mut self) -> &mut RemainingPath;
}

impl ContextGetPathExt for HttpRequestContext {
    // Different implementation needed because we can't produce a Request<Incoming>
    #[cfg(test)]
    fn path(&self) -> &str {
        use crate::state::request_state::RequestState;

        (RequestState::get_from_ctx(self).get::<String>()).expect("must have path (as String)")
    }

    #[cfg(not(test))]
    fn path(&self) -> &str {
        self.request().uri().path()
    }

    fn remaining_path(&mut self) -> &RemainingPath {
        self.create_remaining_path();

        RequestState::get_mut_from_ctx(self).get().unwrap()
    }

    fn remaining_path_mut(&mut self) -> &mut RemainingPath {
        self.create_remaining_path();

        RequestState::get_mut_from_ctx(self).get_mut().unwrap()
    }
}
