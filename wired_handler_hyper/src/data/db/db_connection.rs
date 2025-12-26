use std::{error::Error, future::Future};

use diesel_async::{
    AsyncPgConnection, async_connection_wrapper::AsyncConnectionWrapper,
    pooled_connection::deadpool::Object,
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0}")]
pub enum MigrationError {
    MigrationError(Box<dyn Error + Send + Sync>),
    JoinError(#[from] tokio::task::JoinError),
}

pub type DbConnection = Object<AsyncPgConnection>;

#[doc(hidden)]
pub trait DbConnectionInternalRunMigrationsExt {
    /// Runs pending migrations or shows warning in debug mode
    fn do_run_migrations(
        &mut self,
        migrations: EmbeddedMigrations,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub trait DbConnectionExt: DbConnectionInternalRunMigrationsExt + Send + Sized + 'static {
    /// Runs pending migrations or shows warning in debug mode. For calling in an async context
    fn run_migrations(
        mut self,
        migrations: EmbeddedMigrations,
    ) -> impl Future<Output = Result<(), MigrationError>> {
        async {
            tokio::task::spawn_blocking(move || {
                self.do_run_migrations(migrations)
                    .map_err(MigrationError::MigrationError)
            })
            .await?
        }
    }
}

impl DbConnectionInternalRunMigrationsExt for AsyncConnectionWrapper<DbConnection> {
    /// Runs pending migrations or shows warning in debug mode. If used in an async context, you should prefer `run_migrations`
    ///
    /// # Panics
    /// Panics when directly called in an async context. Use the async version `run_migrations` instead, or use `spawn_blocking`
    #[cfg(debug_assertions)]
    fn do_run_migrations(
        &mut self,
        migrations: EmbeddedMigrations,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let has_pending_migration = self.has_pending_migration(migrations)?;
        if has_pending_migration {
            tracing::warn!("there are pending database migrations");
        } else {
            tracing::debug!("no pending migrations");
        }

        Ok(())
    }

    /// Runs pending migrations or shows warning in debug mode. If used in an async context, you should prefer `run_migrations`
    ///
    /// # Panics
    /// Panics when directly called in an async context. Use the async version `run_migrations` instead, or use `spawn_blocking`
    #[cfg(not(debug_assertions))]
    fn do_run_migrations(
        &mut self,
        migrations: EmbeddedMigrations,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let pending_migrations = self.pending_migrations(migrations)?;
        if pending_migrations.is_empty() {
            tracing::debug!("no pending migrations");
            return Ok(());
        }

        tracing::info!("migrating database");
        self.run_migrations(&pending_migrations)?;
        tracing::info!("migrated database");

        Ok(())
    }
}

impl DbConnectionExt for AsyncConnectionWrapper<DbConnection> {}
