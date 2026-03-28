pub mod ordering;
pub mod schema;
pub mod secret_screen;
pub mod validation;

pub use ordering::EventPosition;
pub use schema::Event;

pub const DEFAULT_SNAPSHOT_INTERVAL_BLOCKS: i64 = 100;
pub const SYSTEM_BOUNDARY_EMITTER_ID_STR: &str = "ffffffff-ffff-7fff-bfff-ffffffffffff";
pub const SYSTEM_BOUNDARY_EMITTER_TITLE: &str = "system_boundary_emitter";

pub fn system_boundary_emitter_id() -> uuid::Uuid {
    uuid::Uuid::parse_str(SYSTEM_BOUNDARY_EMITTER_ID_STR)
        .expect("SYSTEM_BOUNDARY_EMITTER_ID_STR must be a valid UUID")
}

pub fn snapshot_interval_blocks() -> i64 {
    std::env::var("SNAPSHOT_INTERVAL_BLOCKS")
        .ok()
        .and_then(|value| value.trim().parse::<i64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_SNAPSHOT_INTERVAL_BLOCKS)
}
