use std::{collections::HashMap, sync::{Arc, Mutex,RwLock},fmt};
use anyhow::Result;
use anyhow::anyhow;
use dashmap::DashMap;
#[derive(Debug, Clone)]
pub struct CmapMetrics {
    pub data: Arc<DashMap<String, i64>>, //Arc<Mutex<HashMap<String, i64>>> => Arc<DashMap<String, i64>>
}

impl CmapMetrics {
    pub fn new() -> Self {
        CmapMetrics {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
    
}

impl fmt::Display for CmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}