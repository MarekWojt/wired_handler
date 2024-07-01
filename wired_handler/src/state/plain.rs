use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{
    State, StateSyncGet, StateSyncGetCloned, StateSyncMutableGetMut, StateSyncMutableProvide,
};

/// `State` for local usage. Doesn't do anything fancy
#[derive(Debug, Default, State)]
pub struct PlainState(HashMap<TypeId, Box<dyn Any>>);

impl StateSyncGet for PlainState {
    fn get<T: 'static + Send + Sync>(&self) -> Option<impl Deref<Target = T>> {
        self.0
            .get(&TypeId::of::<T>())
            .and_then(|boxed_data| boxed_data.downcast_ref())
    }
}

impl StateSyncMutableGetMut for PlainState {
    fn get_mut<T: 'static + Send + Sync>(&mut self) -> Option<impl DerefMut<Target = T>> {
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

impl StateSyncMutableProvide for PlainState {
    fn provide<T: 'static + Send + Sync>(&mut self, data: T) {
        if let Some(mut current_data) = self.get_mut::<T>() {
            *current_data = data;
            return;
        }
        self.0.insert(TypeId::of::<T>(), Box::new(data));
    }

    fn remove_get<T: 'static + Send + Sync + Clone>(&mut self) -> Option<T> {
        self.0
            .remove(&TypeId::of::<T>())
            .and_then(|val| val.downcast::<T>().ok().map(|d| *d))
    }

    fn remove<T: 'static + Send + Sync>(&mut self) {
        self.0.remove(&TypeId::of::<T>());
    }
}