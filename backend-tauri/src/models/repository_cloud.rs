use std::sync::Arc;

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use tokio::sync::Mutex;

use super::inuse_cloud::InuseCloud;

#[derive(Debug, Clone)]
pub struct RepositoryCloud {
    pub name: String,
    pub inuse_clouds: Arc<Mutex<Vec<InuseCloud>>>,
    pub all_memory: Option<f32>,
    pub inuse_memory: Option<f32>,
}

impl Default for RepositoryCloud {
    fn default() -> Self {
        Self {
            name: "".into(),
            inuse_clouds: Arc::new(Mutex::new(vec![])),
            all_memory: None,
            inuse_memory: None,
        }
    }
}

impl RepositoryCloud {
    pub fn new(name: String, inuse_clouds: Arc<Mutex<Vec<InuseCloud>>>) -> Self {
        Self {
            name,
            inuse_clouds,
            ..Default::default()
        }
    }
}

impl Serialize for RepositoryCloud {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let inuse_clouds = self
            .inuse_clouds
            .try_lock()
            .expect("Failed to lock mutex for serialization");

        let mut state = serializer.serialize_struct("RepositoryCloud", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("inuse_clouds", &*inuse_clouds)?;
        state.serialize_field("all_memory", &self.all_memory)?;
        state.serialize_field("inuse_memory", &self.inuse_memory)?;
        state.end()
    }
}
