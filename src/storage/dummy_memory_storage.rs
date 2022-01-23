use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;
use bytes::Bytes;

use crate::error::Result;
use crate::storage::Storage;
use crate::value::{Entry, Request};

pub struct DummyMemoryStorage(RwLock<HashMap<Bytes, Entry>>);

impl DummyMemoryStorage {
    pub fn new() -> Self {
        DummyMemoryStorage(RwLock::default())
    }
}

impl Default for DummyMemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Storage for DummyMemoryStorage {
    async fn put(&self, request: Request) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        for entry in request.entries.into_iter() {
            writer.insert(entry.clone().key, entry);
        }
        Ok(())
    }

    async fn get(&self, key: &Bytes) -> Result<Option<Entry>> {
        Ok(self.0.read().unwrap().get(key).cloned())
    }
}
