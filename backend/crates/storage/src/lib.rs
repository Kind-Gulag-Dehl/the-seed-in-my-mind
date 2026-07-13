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
use encoding::payload::{canonical_json_payload_hash_hex, payload_hash_hex};
use event_log::validation::{validate_event, validate_stage0_internal_event};
use event_log::{system_boundary_emitter_id, Event};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Transaction};
use uuid::Uuid;
use verification::signatures::{
    authored_candidate_hash_v0, decode_signature64, public_key_ref_v0, signed_candidate_bytes_v0,
    verify_ed25519_v0, AuthoredEventCandidate, PAYLOAD_BINDING_EMBEDDED,
};

use db::*;
