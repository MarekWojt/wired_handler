use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{
    State, StateSyncGet, StateSyncGetCloned, StateSyncMutableGetMut,
    StateSyncMutableGetMutOrInsert, StateSyncMutableInsert, StateSyncMutableRemoveGet,
};

/// `State` for local usage. Doesn't do anything fancy
#[derive(Debug, Default, State)]
pub struct PlainState(HashMap<TypeId, Box<dyn Any + Send + Sync>>);

impl StateSyncGet for PlainState {
    fn get<T: 'static + Send + Sync>(&self) -> Option<&T> {
        self.0
            .get(&TypeId::of::<T>())
            .and_then(|boxed_data| boxed_data.downcast_ref())
    }

    fn exists<T: 'static + Send + Sync>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<T>())
    }
}

impl StateSyncMutableGetMut for PlainState {
    fn get_mut<T: 'static + Send + Sync>(&mut self) -> Option<&mut T> {
        self.0
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed_data| boxed_data.downcast_mut())
    }
}

impl StateSyncGetCloned for PlainState {
    fn get_cloned<T: 'static + Send + Sync + Clone>(&self) -> Option<T> {
        self.0
            .get(&TypeId::of::<T>())
            .and_then(|boxed_data| boxed_data.downcast_ref())
            .cloned()
    }
}

impl StateSyncMutableInsert for PlainState {
    fn insert<T: 'static + Send + Sync>(&mut self, data: T) {
        if let Some(current_data) = self.get_mut::<T>() {
            *current_data = data;
            return;
        }
        self.0.insert(TypeId::of::<T>(), Box::new(data));
    }

    fn remove<T: 'static + Send + Sync>(&mut self) {
        self.0.remove(&TypeId::of::<T>());
    }
}

impl StateSyncMutableRemoveGet for PlainState {
    fn remove_get<T: 'static + Send + Sync>(&mut self) -> Option<T> {
        self.0
            .remove(&TypeId::of::<T>())
            .and_then(|val| val.downcast::<T>().ok().map(|d| *d))
    }
}

impl StateSyncMutableGetMutOrInsert for PlainState {
    fn get_mut_or_insert_with<T: 'static + Send + Sync>(
        &mut self,
        get_data: impl FnOnce() -> T,
    ) -> &mut T {
        if !self.exists::<T>() {
            self.insert(get_data());
        }

        self.get_mut().unwrap() // we just made sure it exists
    }
}
