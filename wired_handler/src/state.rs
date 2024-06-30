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

    /// Get data synchronously from `State` `&self`
    pub trait StateSyncGet: State {
        fn get<T: 'static + Send + Sync>(&self) -> Option<impl Deref<Target = T>>;
    }

    /// Get mutable data synchronously from `State` `&self`
    pub trait StateSyncGetMut: State {
        fn get_mut<T: 'static + Send + Sync>(&self) -> Option<impl DerefMut<Target = T>>;
    }

    /// Get cloned data synchronously from `State` `&self`
    pub trait StateSyncGetCloned: State {
        fn get_cloned<T: 'static + Send + Sync + Clone>(&self) -> Option<T>;
    }

    /// Provide or remove data synchronously from `State` `&self`
    pub trait StateSyncProvide: State {
        fn provide<T: 'static + Send + Sync>(&self, data: T);
        fn remove<T: 'static + Send + Sync>(&self);
        fn remove_get<T: 'static + Send + Sync + Clone>(&self) -> Option<T>;
    }
}

mod sync_mutable {
    use std::ops::DerefMut;

    use super::State;

    /// Get mutable data synchronously from `State` `&mut self`
    pub trait StateSyncMutableGetMut: State {
        fn get_mut<T: 'static + Send + Sync>(&mut self) -> Option<impl DerefMut<Target = T>>;
    }

    /// Provide or remove data synchronously from `State` `&mut self`
    pub trait StateSyncMutableProvide: State {
        fn provide<T: 'static + Send + Sync>(&mut self, data: T);
        fn remove<T: 'static + Send + Sync>(&mut self);
        fn remove_get<T: 'static + Send + Sync + Clone>(&mut self) -> Option<T>;
    }
}

mod async_immutable {
    use std::ops::{Deref, DerefMut};

    use super::State;

    /// Get data asynchronously from `State` `&self`
    pub trait StateAsyncGet: State {
        fn get<T: 'static + Send + Sync>(
            &self,
        ) -> impl std::future::Future<Output = Option<impl Deref<Target = T>>> + Send;
    }

    /// Get mutable data asynchronously from `State` `&self`
    pub trait StateAsyncGetMut: State {
        fn get_mut<T: 'static + Send + Sync>(
            &self,
        ) -> impl std::future::Future<Output = Option<impl DerefMut<Target = T>>> + Send;
    }

    /// Get cloned data asynchronously from `State` `&self`
    pub trait StateAsyncGetCloned: State {
        fn get_cloned<T: 'static + Send + Sync + Clone>(
            &self,
        ) -> impl std::future::Future<Output = Option<T>> + Send;
    }

    /// Provide or remove data synchronously from `State` `&self`
    pub trait StateAsyncProvide: State {
        fn provide<T: 'static + Send + Sync>(
            &self,
            data: T,
        ) -> impl std::future::Future<Output = ()> + Send;
        fn remove<T: 'static + Send + Sync>(&self) -> impl std::future::Future<Output = ()> + Send;
        fn remove_get<T: 'static + Send + Sync + Clone>(
            &self,
        ) -> impl std::future::Future<Output = Option<T>> + Send;
    }
}

mod async_mutable {
    use std::ops::DerefMut;

    use super::State;

    /// Get mutable data asynchronously from `State` `&mut self`
    pub trait StateAsyncMutableGetMut: State {
        fn get_mut<T: 'static + Send + Sync>(
            &mut self,
        ) -> impl std::future::Future<Output = Option<impl DerefMut<Target = T>>> + Send;
    }

    /// Provide or remove data asynchronously from `State` `&mut self`
    pub trait StateAsyncMutableProvide: State {
        fn provide<T: 'static + Send + Sync>(
            &mut self,
            data: T,
        ) -> impl std::future::Future<Output = ()> + Send;
        fn remove<T: 'static + Send + Sync + Clone>(
            &mut self,
        ) -> impl std::future::Future<Output = ()> + Send;
        fn remove_get<T: 'static + Send + Sync + Clone>(
            &mut self,
        ) -> impl std::future::Future<Output = Option<T>> + Send;
    }
}
