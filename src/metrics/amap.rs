use std::{collections::HashMap, ffi::os_str::Display, fmt, sync::{Arc, atomic::{AtomicI64, Ordering}}};
use anyhow::Result;
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(metric_names:  &[&'static str]) -> Self {
        let map = metric_names.iter().map(|&name| (name, AtomicI64::new(0))).collect();
        AmapMetrics {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let counter = self.data.get(key.as_ref()).ok_or_else(|| anyhow::anyhow!("Key not found"))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}



impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        AmapMetrics {
            data: Arc::clone(&self.data),
        }
    }
}

impl fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (key, counter) in self.data.iter() {
            writeln!(f, "{}: {}", key, counter.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}