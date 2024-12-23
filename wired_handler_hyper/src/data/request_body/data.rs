use serde::de::DeserializeOwned;

/// For storing the body in the `RequestState`
#[derive(Debug, Clone)]
pub(super) struct RequestBody<T: DeserializeOwned>(T);

/// Marker struct, inserted after body generation
#[derive(Debug)]
pub(super) struct RequestBodyParsed;

impl<T: DeserializeOwned> RequestBody<T> {
    pub fn new(data: T) -> Self {
        Self(data)
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}
