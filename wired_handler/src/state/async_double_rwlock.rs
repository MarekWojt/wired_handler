use crate::{
    State, StateAsyncGet, StateAsyncGetCloned, StateAsyncGetMut, StateAsyncGetMutOrInsert,
    StateAsyncInsert, StateAsyncRemoveGetCloned,
};

use std::{
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
    sync::Arc,
};

use rustc_hash::FxHashMap;
use tokio::sync::{
    OwnedRwLockMappedWriteGuard, OwnedRwLockReadGuard, OwnedRwLockWriteGuard, RwLock,
};

type SyncedAny = Arc<RwLock<dyn Any + Send + Sync>>;

/// `State` to be shared between async tasks. Utilizes `tokio`'s `RwLock` for concurrency
#[derive(Debug, Clone, Default, State)]
pub struct AsyncDoubleRwLockState(Arc<RwLock<FxHashMap<TypeId, SyncedAny>>>);

impl AsyncDoubleRwLockState {
    async fn internal_get_mut<T: 'static + Send + Sync>(
        &self,
    ) -> Option<OwnedRwLockMappedWriteGuard<(dyn Any + Send + Sync), T>> {
        let data = { self.0.read().await.get(&TypeId::of::<T>())?.clone() };

        let guard = data.write_owned().await;

        OwnedRwLockWriteGuard::try_map(guard, |data| data.downcast_mut::<T>()).ok()
    }
}

impl StateAsyncGet for AsyncDoubleRwLockState {
    async fn get<T: 'static + Send + Sync>(&self) -> Option<impl Deref<Target = T>> {
        let data = { self.0.read().await.get(&TypeId::of::<T>())?.clone() };

        let guard = data.read_owned().await;

        OwnedRwLockReadGuard::try_map(guard, |data| data.downcast_ref::<T>()).ok()
    }

    async fn exists<T: 'static + Send + Sync>(&self) -> bool {
        self.0.read().await.contains_key(&TypeId::of::<T>())
    }
}

impl StateAsyncGetMut for AsyncDoubleRwLockState {
    async fn get_mut<T: 'static + Send + Sync>(&self) -> Option<impl DerefMut<Target = T>> {
        self.internal_get_mut().await
    }
}

impl StateAsyncGetCloned for AsyncDoubleRwLockState {
    async fn get_cloned<T: 'static + Send + Sync + Clone>(&self) -> Option<T> {
        self.get::<T>().await.map(|data| data.clone())
    }
}

impl StateAsyncInsert for AsyncDoubleRwLockState {
    async fn insert<T: 'static + Send + Sync>(&self, data: T) {
        if let Some(mut current_data) = self.get_mut::<T>().await {
            *current_data = data;
            return;
        }
        self.0
            .write()
            .await
            .insert(TypeId::of::<T>(), Arc::new(RwLock::new(data)));
    }

    async fn remove<T: 'static + Send + Sync>(&self) {
        self.0.write().await.remove(&TypeId::of::<T>());
    }
}

impl StateAsyncRemoveGetCloned for AsyncDoubleRwLockState {
    async fn remove_get_cloned<T: 'static + Send + Sync + Clone>(&self) -> Option<T> {
        let data_arc = self.0.write().await.remove(&TypeId::of::<T>())?;

        /*
        Some(match Arc::try_unwrap(data_arc) {
            Ok(rwlock_data) => rwlock_data.into_inner(),
            Err(arc_data) => T::clone(&*arc_data.read().await),
        })
         */

        let cloned_data = T::clone(data_arc.read_owned().await.downcast_ref()?);

        Some(cloned_data)
    }
}

impl StateAsyncGetMutOrInsert for AsyncDoubleRwLockState {
    async fn get_mut_or_insert_with<T: 'static + Send + Sync>(
        &self,
        get_data: impl FnOnce() -> T + std::marker::Send,
    ) -> impl DerefMut<Target = T> {
        if let Some(current_data) = self.internal_get_mut::<T>().await {
            return current_data;
        }

        let inserted_data = Arc::new(RwLock::new(get_data())) as Arc<RwLock<dyn Any + Send + Sync>>;
        {
            self.0
                .write()
                .await
                .insert(TypeId::of::<T>(), inserted_data.clone());
        }

        OwnedRwLockWriteGuard::map(inserted_data.write_owned().await, |data| {
            data.downcast_mut().unwrap() // wrong types cannot be inserted
        })
    }
}
