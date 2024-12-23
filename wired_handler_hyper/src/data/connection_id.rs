use uuid::Uuid;

/// Identifies a connection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionId(Uuid);

impl From<Uuid> for ConnectionId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl ConnectionId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn uuid(&self) -> &Uuid {
        &self.0
    }
}
