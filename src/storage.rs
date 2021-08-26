mod dummy_memory_storage;

pub use dummy_memory_storage::DummyMemoryStorage;

use async_trait::async_trait;
use bytes::Bytes;

use crate::error::Result;
use crate::value::{Entry, Request};

#[async_trait]
pub trait Storage {
    async fn put(&self, request: Request) -> Result<()>;
    async fn get(&self, key: &Bytes) -> Result<Option<Entry>>;
}
