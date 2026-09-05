#![cfg(feature = "json")]
#![warn(missing_debug_implementations)]
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate, clippy::must_use_unit, clippy::struct_field_names)]

pub mod data;
mod http;
pub mod prelude;
pub mod state;

pub use http::*;
