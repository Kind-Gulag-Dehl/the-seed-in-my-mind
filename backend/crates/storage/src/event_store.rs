use common::Result;
use event_log::Event;

pub trait EventStore {
    fn append(&self, event: &Event) -> Result<()>;
}
