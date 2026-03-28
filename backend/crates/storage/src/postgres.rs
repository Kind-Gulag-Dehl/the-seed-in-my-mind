use crate::{event_store::EventStore, snapshot_store::SnapshotStore};
use common::Error;
use common::Result;
use event_log::Event;
use snapshot::SnapshotFormat;

#[derive(Debug, Default)]
pub struct PostgresStore;

impl EventStore for PostgresStore {
    fn append(&self, _event: &Event) -> Result<()> {
        Err(Error::Placeholder(
            "storage::postgres::PostgresStore::append is stage1 experimental".to_string(),
        ))
    }
}

impl SnapshotStore for PostgresStore {
    fn put(&self, _snapshot: &SnapshotFormat) -> Result<()> {
        Err(Error::Placeholder(
            "storage::postgres::PostgresStore::put is stage1 experimental".to_string(),
        ))
    }
}
