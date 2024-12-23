use serde::{Deserialize, Serialize};

/// The bind config for the HTTP part
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BindConfig {
    #[serde(default)]
    pub addr: String,
}

impl Default for BindConfig {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:8000".into(),
        }
    }
}

#[cfg(feature = "diesel")]
/// The db config for the database part
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DbConfig {
    #[serde(default)]
    pub addr: String,
}
