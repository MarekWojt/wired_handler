use std::future::Future;

use diesel_async::pooled_connection::deadpool::PoolError;

use crate::{
    prelude::*,
    state::{context::HttpRequestContext, global_state::GlobalState},
};

pub use db_connection::*;
pub use db_pool::*;

mod db_connection;
mod db_pool;

/// For getting a database connection from the pool
pub trait ContextGetDbExt {
    /// Gets a database connection from the pool
    fn db(&self) -> impl Future<Output = Result<DbConnection, PoolError>>;
}

impl ContextGetDbExt for HttpRequestContext {
    async fn db(&self) -> Result<DbConnection, PoolError> {
        let db_pool = GlobalState::get_from_ctx(self)
            .get_cloned::<DbPool>()
            .await
            .expect("DbPool must be inserted");

        let db = db_pool.get().await?;
        Ok(db)
    }
}
