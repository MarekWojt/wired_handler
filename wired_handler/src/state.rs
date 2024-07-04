pub mod async_double_rwlock;
pub mod plain;

pub use async_immutable::*;
pub use async_mutable::*;
pub use sync_immutable::*;
pub use sync_mutable::*;

/// Marks an object as state
pub trait State {}

mod sync_immutable {
    use std::ops::{Deref, DerefMut};

    use super::State;

    /// Get data immutably (sync immutable version)
    pub trait StateSyncGet: State {
        /// Gets data of type `T`
        fn get<T: 'static + Send + Sync>(&self) -> Option<impl Deref<Target = T>>;
    }

    /// Get data mutably (sync immutable version)
    pub trait StateSyncGetMut: State {
        /// Gets data of type `T` mutably
        fn get_mut<T: 'static + Send + Sync>(&self) -> Option<impl DerefMut<Target = T>>;
    }

    /// Get cloned data (sync immutable version)
    pub trait StateSyncGetCloned: State {
        /// Gets a clone of the data of type `T`
        fn get_cloned<T: 'static + Send + Sync + Clone>(&self) -> Option<T>;
    }

    /// Insert and remove data (sync immutable version)
    pub trait StateSyncInsert: State {
        /// Inserts `data` as data of type `T`
        fn insert<T: 'static + Send + Sync>(&self, data: T);
        /// Removes data of type `T`
        fn remove<T: 'static + Send + Sync>(&self);
    }

    /// Remove data and return a clone (sync immutable version)
    pub trait StateSyncRemoveGetCloned: State {
        /// Removes and returns data of type `T`, cloned if necessary
        fn remove_get_cloned<T: 'static + Send + Sync + Clone>(&self) -> Option<T>;
    }

    /// Remove data and return it (sync immutable version)
    pub trait StateSyncRemoveGet: State {
        /// Removes and returns data of type `T`
        fn remove_get<T: 'static + Send + Sync>(&self) -> Option<T>;
    }

    /// Get data mutably or insert (sync immutable version)
    pub trait StateSyncGetMutOrInsert: State {
        /// Returns data of type `T` mutably, inserts if not found
        fn get_mut_or_insert_with<T: 'static + Send + Sync>(
            &self,
            get_data: impl FnOnce() -> T,
        ) -> impl DerefMut<Target = T>;
        /// Returns data of type `T` mutably, inserts if not found
        fn get_mut_or_insert<T: 'static + Send + Sync>(
            &self,
            data: T,
        ) -> impl DerefMut<Target = T> {
            self.get_mut_or_insert_with(move || data)
        }
        /// Returns data of type `T` mutably, inserts default if not found
        fn get_mut_or_insert_default<T: 'static + Send + Sync + Default>(
            &self,
        ) -> impl DerefMut<Target = T> {
            self.get_mut_or_insert_with(|| T::default())
        }
    }
}

mod sync_mutable {
    use std::ops::DerefMut;

    use super::State;

    /// Get data mutably (sync mutable version)
    pub trait StateSyncMutableGetMut: State {
        /// Gets data of type `T` mutably
        fn get_mut<T: 'static + Send + Sync>(&mut self) -> Option<impl DerefMut<Target = T>>;
    }

    /// Insert and remove data (sync mutable version)
    pub trait StateSyncMutableInsert: State {
        /// Inserts `data` as data of type `T`
        fn insert<T: 'static + Send + Sync>(&mut self, data: T);
        /// Removes data of type `T`
        fn remove<T: 'static + Send + Sync>(&mut self);
    }

    /// Remove data and return a clone (sync mutable version)
    pub trait StateSyncMutableRemoveGetCloned: State {
        /// Removes and returns data of type `T`, cloned if necessary
        fn remove_get_cloned<T: 'static + Send + Sync + Clone>(&mut self) -> Option<T>;
    }

    /// Remove data and return it (sync mutable version)
    pub trait StateSyncMutableRemoveGet: State {
        /// Removes and returns data of type `T`
        fn remove_get<T: 'static + Send + Sync>(&mut self) -> Option<T>;
    }

    /// Get data mutably or insert (sync mutable version)
    pub trait StateSyncMutableGetMutOrInsert: State {
        /// Returns data of type `T` mutably, inserts if not found
        fn get_mut_or_insert_with<T: 'static + Send + Sync>(
            &mut self,
            get_data: impl FnOnce() -> T,
        ) -> impl DerefMut<Target = T>;
        /// Returns data of type `T` mutably, inserts if not found
        fn get_mut_or_insert<T: 'static + Send + Sync>(
            &mut self,
            data: T,
        ) -> impl DerefMut<Target = T> {
            self.get_mut_or_insert_with(move || data)
        }
        /// Returns data of type `T` mutably, inserts default if not found
        fn get_mut_or_insert_default<T: 'static + Send + Sync + Default>(
            &mut self,
        ) -> impl DerefMut<Target = T> {
            self.get_mut_or_insert_with(|| T::default())
        }
    }
}

mod async_immutable {
    use std::ops::{Deref, DerefMut};

    use super::State;

    /// Get data immutably (async immutable version)
    pub trait StateAsyncGet: State {
        /// Gets data of type `T`
        fn get<T: 'static + Send + Sync>(
            &self,
        ) -> impl std::future::Future<Output = Option<impl Deref<Target = T>>> + Send;
    }

    /// Get data mutably (async immutable version)
    pub trait StateAsyncGetMut: State {
        /// Gets data of type `T` mutably
        fn get_mut<T: 'static + Send + Sync>(
            &self,
        ) -> impl std::future::Future<Output = Option<impl DerefMut<Target = T>>> + Send;
    }

    /// Get cloned data (async immutable version)
    pub trait StateAsyncGetCloned: State {
        /// Gets a clone of the data of type `T`
        fn get_cloned<T: 'static + Send + Sync + Clone>(
            &self,
        ) -> impl std::future::Future<Output = Option<T>> + Send;
    }

    /// Insert and remove data (async immutable version)
    pub trait StateAsyncInsert: State {
        /// Inserts `data` as data of type `T`
        fn insert<T: 'static + Send + Sync>(
            &self,
            data: T,
        ) -> impl std::future::Future<Output = ()> + Send;
        /// Removes data of type `T`
        fn remove<T: 'static + Send + Sync>(&self) -> impl std::future::Future<Output = ()> + Send;
    }

    /// Remove data and return a clone (async immutable version)
    pub trait StateAsyncRemoveGetCloned: State {
        /// Removes and returns data of type `T`, cloned if necessary
        fn remove_get_cloned<T: 'static + Send + Sync + Clone>(
            &self,
        ) -> impl std::future::Future<Output = Option<T>> + Send;
    }

    /// Remove data and return it (async immutable version)
    pub trait StateAsyncRemoveGet: State {
        /// Removes and returns data of type `T`
        fn remove_get<T: 'static + Send + Sync>(
            &self,
        ) -> impl std::future::Future<Output = Option<T>> + Send;
    }

    /// Get data mutably or insert (async immutable version)
    pub trait StateAsyncGetMutOrInsert: State {
        /// Returns data of type `T` mutably, inserts if not found
        fn get_mut_or_insert_with<T: 'static + Send + Sync>(
            &self,
            get_data: impl FnOnce() -> T + std::marker::Send,
        ) -> impl std::future::Future<Output = impl DerefMut<Target = T>> + Send;
        /// Returns data of type `T` mutably, inserts if not found
        fn get_mut_or_insert<T: 'static + Send + Sync>(
            &self,
            data: T,
        ) -> impl std::future::Future<Output = impl DerefMut<Target = T>> + Send {
            self.get_mut_or_insert_with(move || data)
        }
        /// Returns data of type `T` mutably, inserts default if not found
        fn get_mut_or_insert_default<T: 'static + Send + Sync + Default>(
            &self,
        ) -> impl std::future::Future<Output = impl DerefMut<Target = T>> + Send {
            self.get_mut_or_insert_with(|| T::default())
        }
    }
}

mod async_mutable {
    use std::ops::DerefMut;

    use super::State;

    /// Get data mutably (async mutable version)
    pub trait StateAsyncMutableGetMut: State {
        /// Gets data of type `T` mutably
        fn get_mut<T: 'static + Send + Sync>(
            &mut self,
        ) -> impl std::future::Future<Output = Option<impl DerefMut<Target = T>>> + Send;
    }

    /// Insert and remove data (async mutable version)
    pub trait StateAsyncMutableInsert: State {
        /// Inserts `data` as data of type `T`
        fn insert<T: 'static + Send + Sync>(
            &mut self,
            data: T,
        ) -> impl std::future::Future<Output = ()> + Send;
        /// Removes data of type `T`
        fn remove<T: 'static + Send + Sync + Clone>(
            &mut self,
        ) -> impl std::future::Future<Output = ()> + Send;
    }

    /// Remove data and return a clone (async mutable version)
    pub trait StateAsyncMutableRemoveGetCloned: State {
        /// Removes and returns data of type `T`, cloned if necessary
        fn remove_get_cloned<T: 'static + Send + Sync + Clone>(
            &mut self,
        ) -> impl std::future::Future<Output = Option<T>> + Send;
    }

    /// Remove data and return it (async mutable version)
    pub trait StateAsyncMutableRemoveGet: State {
        /// Removes and returns data of type `T`
        fn remove_get<T: 'static + Send + Sync>(
            &mut self,
        ) -> impl std::future::Future<Output = Option<T>> + Send;
    }

    /// Get data mutably or insert (async mutable version)
    pub trait StateAsyncMutableGetMutOrInsert: State {
        /// Returns data of type `T` mutably, inserts if not found
        fn get_mut_or_insert_with<T: 'static + Send + Sync>(
            &mut self,
            get_data: impl FnOnce() -> T + std::marker::Send,
        ) -> impl std::future::Future<Output = impl DerefMut<Target = T>> + Send;
        /// Returns data of type `T` mutably, inserts if not found
        fn get_mut_or_insert<T: 'static + Send + Sync>(
            &mut self,
            data: T,
        ) -> impl std::future::Future<Output = impl DerefMut<Target = T>> + Send {
            self.get_mut_or_insert_with(move || data)
        }
        /// Returns data of type `T` mutably, inserts default if not found
        fn get_mut_or_insert_default<T: 'static + Send + Sync + Default>(
            &mut self,
        ) -> impl std::future::Future<Output = impl DerefMut<Target = T>> + Send {
            self.get_mut_or_insert_with(|| T::default())
        }
    }
}
