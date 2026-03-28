pub mod event_store;
pub mod postgres;
mod queries;
pub mod snapshot_store;

mod accounts;
mod canonical;
mod db;
mod private_data;
mod read;
mod types;

#[cfg(test)]
mod tests;

pub use accounts::ensure_session_hmac_key_ready;
pub use event_log::secret_screen::screen_text_for_secrets;
pub use types::*;

use anyhow::{anyhow, Result};
use encoding::hash::hash_bytes;
use encoding::payload::payload_hash_hex;
use event_log::validation::validate_event;
use event_log::{system_boundary_emitter_id, Event};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Transaction};
use uuid::Uuid;

use db::*;
