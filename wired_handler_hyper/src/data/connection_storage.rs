use std::collections::HashMap;

use crate::state::connection_state::ConnectionState;

use super::connection_id::ConnectionId;

/// Stores all connections for a state, identified by `ConnectionId`
#[derive(Debug, Default)]
pub struct ConnectionStorage(HashMap<ConnectionId, ConnectionState>);

impl ConnectionStorage {
    pub fn get(&self) -> &HashMap<ConnectionId, ConnectionState> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut HashMap<ConnectionId, ConnectionState> {
        &mut self.0
    }
}
