use serde::de::DeserializeOwned;

#[derive(Debug)]
pub(super) struct QueryParams<T: DeserializeOwned + Send + Sync + 'static>(Option<T>);

impl<T: DeserializeOwned + Send + Sync + 'static> QueryParams<T> {
    pub fn new(data: Option<T>) -> Self {
        Self(data)
    }

    pub fn get(&self) -> Option<&T> {
        self.0.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.0.as_mut()
    }

    pub fn into_inner(self) -> Option<T> {
        self.0
    }
}
