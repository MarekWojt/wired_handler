use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

/// Holds some state by types
#[derive(Debug, Default)]
pub struct StateHolder {
    data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl StateHolder {
    /// Inserts data by type
    pub fn provide<T: Send + Sync + 'static>(&mut self, data: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(data));
    }

    /// Removes data by type and returns it
    pub fn remove<T: Send + Sync + 'static>(&mut self) -> Option<T> {
        self.data
            .remove(&TypeId::of::<T>())
            .and_then(|val| match val.downcast::<T>().ok() {
                Some(d) => Some(*d),
                None => {
                    log::warn!("state conversion failed, corrupt data?");
                    None
                }
            })
    }

    /// Return data by type
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.data
            .get(&TypeId::of::<T>())
            .and_then(|val| val.downcast_ref::<T>())
    }
}
