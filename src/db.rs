use bytes::Bytes;

use crate::{
    error::Result,
    storage::Storage,
    value::{Entry, Request},
};

#[derive(Debug)]
pub struct Potato<S>
where
    S: Storage,
{
    storage: S,
}

impl<S> Potato<S>
where
    S: Storage,
{
    pub fn new(storage: S) -> Self {
        Potato { storage }
    }

    pub async fn put(&self, request: Request) -> Result<()> {
        self.storage.put(request).await
    }

    pub async fn get(&self, key: &Bytes) -> Result<Option<Entry>> {
        self.storage.get(key).await
    }
}

#[cfg(test)]
mod test {
    use bytes::Bytes;

    use super::Potato;
    use crate::{
        storage::DummyMemoryStorage,
        value::{Entry, Request},
    };

    #[tokio::test]
    async fn test_basic_db_with_dummy_memory_storage() {
        let storage = DummyMemoryStorage::default();
        let potato = Potato::new(storage);

        let key = Bytes::from("Hello, world!");
        let value = Bytes::from("👋, 🌍!");
        assert!(potato
            .put(Request {
                entries: vec![Entry {
                    key: key.clone(),
                    value: value.clone(),
                }],
            })
            .await
            .is_ok());
        let entry = potato.get(&key).await.unwrap().unwrap();
        assert_eq!(value, entry.value);
        assert!(potato
            .get(&Bytes::from("Not found"))
            .await
            .unwrap()
            .is_none());
    }
}
