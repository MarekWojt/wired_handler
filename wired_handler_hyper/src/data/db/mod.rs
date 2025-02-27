use std::future::Future;

use diesel_async::{
    async_connection_wrapper::AsyncConnectionWrapper,
    pooled_connection::{
        deadpool::{BuildError, Pool, PoolError},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};
use diesel_migrations::EmbeddedMigrations;
use thiserror::Error;
use wired_handler::Handler;

use crate::{
    prelude::*,
    state::{
        context::{HttpRequestContext, SessionlessRequestContext},
        global_state::GlobalState,
    },
};

pub use db_connection::*;
pub use db_pool::*;

use super::config::DbConfig;

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

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LoadDbError {
    #[error("{0}")]
    DbPool(#[from] BuildError),
    #[error("{0}")]
    MigrationDbPool(#[from] PoolError),
    #[error("{0}")]
    MigrationError(#[from] MigrationError),
}

pub trait LoadDbExt {
    /// Loads the database and applies migrations
    ///
    /// In debug mode, only generates a warning if there are pending migrations
    fn load_db(
        &self,
        db_config: DbConfig,
        migrations: impl Into<Option<EmbeddedMigrations>>,
    ) -> impl Future<Output = Result<(), LoadDbError>>;
}

impl<F: Future<Output = HttpRequestContext> + 'static + Send> LoadDbExt
    for Handler<SessionlessRequestContext, HttpRequestContext, GlobalState, F>
{
    async fn load_db(
        &self,
        db_config: DbConfig,
        migrations: impl Into<Option<EmbeddedMigrations>>,
    ) -> Result<(), LoadDbError> {
        let db_addr = &db_config.addr;
        let db_pool: DbPool = {
            let config: AsyncDieselConnectionManager<AsyncPgConnection> =
                AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_addr);
            Pool::builder(config).build()?
        };

        if let Some(migrations) = migrations.into() {
            let conn = db_pool.get().await?;
            AsyncConnectionWrapper::from(conn)
                .run_migrations(migrations)
                .await?;
        }

        {
            let global_state = self.state();

            global_state.insert(db_config).await;
            global_state.insert(db_pool).await;
        }

        Ok(())
    }
}
