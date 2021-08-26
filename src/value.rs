use bytes::Bytes;

#[derive(Clone)]
pub struct Entry {
    pub key: Bytes,
    pub value: Bytes,
}

#[derive(Clone)]
pub struct Request {
    pub entries: Vec<Entry>,
}

// TODO: add a marco to create a `Request` easily.
