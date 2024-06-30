use crate::{State, StateAsyncGet, StateAsyncGetCloned, StateAsyncGetMut, StateAsyncProvide};

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::Deref,
    sync::Arc,
};

use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard, RwLock};

type SyncedAny = Arc<RwLock<dyn Any + Send + Sync>>;

/// `State` to be shared between async tasks. Utilizes `tokio`'s `RwLock` for concurrency
#[derive(Debug, Clone, Default, State)]
pub struct AsyncDoubleRwLockState(Arc<RwLock<HashMap<TypeId, SyncedAny>>>);

impl StateAsyncGet for AsyncDoubleRwLockState {
    async fn get<T: 'static + Send + Sync>(&self) -> Option<impl Deref<Target = T>> {
        let data = { self.0.read().await.get(&TypeId::of::<T>())?.clone() };

        let guard = data.read_owned().await;

        OwnedRwLockReadGuard::try_map(guard, |data| data.downcast_ref::<T>()).ok()
    }
}

impl StateAsyncGetMut for AsyncDoubleRwLockState {
    async fn get_mut<T: 'static + Send + Sync>(
        &self,
    ) -> Option<impl std::ops::DerefMut<Target = T>> {
        let data = { self.0.read().await.get(&TypeId::of::<T>())?.clone() };

        let guard = data.write_owned().await;

        OwnedRwLockWriteGuard::try_map(guard, |data| data.downcast_mut::<T>()).ok()
    }
}

impl StateAsyncGetCloned for AsyncDoubleRwLockState {
    async fn get_cloned<T: 'static + Send + Sync + Clone>(&self) -> Option<T> {
        let data = { self.0.read().await.get(&TypeId::of::<T>())?.clone() };

        let guarded_data = data.read().await;

        guarded_data.downcast_ref().cloned()
    }
}

impl StateAsyncProvide for AsyncDoubleRwLockState {
    async fn provide<T: 'static + Send + Sync>(&self, data: T) {
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

    async fn remove_get<T: 'static + Send + Sync + Clone>(&self) -> Option<T> {
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
