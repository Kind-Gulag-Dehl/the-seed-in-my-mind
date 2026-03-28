use common::Result;
use snapshot::SnapshotFormat;

pub trait SnapshotStore {
    fn put(&self, snapshot: &SnapshotFormat) -> Result<()>;
}
