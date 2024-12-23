use diesel_async::{
    pooled_connection::{
        deadpool::{self, Pool},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};

use crate::data::config::DbConfig;

pub type DbPool = Pool<AsyncPgConnection>;

pub trait DbPoolExt: Sized {
    fn new(db_config: &DbConfig) -> Result<Self, deadpool::BuildError>;
}

impl DbPoolExt for DbPool {
    fn new(db_config: &DbConfig) -> Result<Self, deadpool::BuildError> {
        let config: AsyncDieselConnectionManager<AsyncPgConnection> =
            AsyncDieselConnectionManager::<AsyncPgConnection>::new(&db_config.addr);
        Pool::builder(config).build()
    }
}
