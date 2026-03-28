use anyhow::{anyhow, Context, Result};
use encoding::canonical::{encode_u16, validate_id};
use encoding::hash::hash_with_domain;
use replay::ReplayDriver;
use snapshot::{
    compute_title_sentence_payload_root, sha256_hex, to_hex, CONNECTIONS_SECTION_ID,
    IDEAS_SECTION_ID, RAILS_SECTION_ID,
};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, FromRow)]
struct SnapshotMeta {
    block_height: i64,
    snapshot_hash: String,
    state_root_hash: Option<String>,
    title_sentence_payload_root: Option<String>,
    shared_map_commitment: Option<String>,
    active_rulebook_set_hash: Option<String>,
    artifact_path: Option<String>,
    artifact_sha256: Option<String>,
    last_event_id: Option<Uuid>,
    event_count: Option<i64>,
}

#[derive(Debug)]
enum Selection {
    Latest,
    Height(i64),
    SnapshotHash(String),
}

#[derive(Debug, Clone, Copy)]
enum Profile {
    Stage0,
}

#[derive(Debug)]
struct ParsedSnapshot {
    format_version: u16,
    header_flags: u16,
    block_height: u64,
    snapshot_kind: u8,
    snapshot_tier_id: String,
    last_event_id: String,
    event_count: u64,
    active_rulebook_set_hash: Vec<u8>,
    state_root_hash: Vec<u8>,
    title_sentence_payload_root: Vec<u8>,
    sections: Vec<ParsedSection>,
}

#[derive(Debug)]
struct ParsedSection {
    id: u16,
    item_count: u32,
    byte_len: u64,
    hash: Vec<u8>,
    bytes: Vec<u8>,
}

struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    fn read_exact(&mut self, len: usize) -> Result<&'a [u8]> {
        let end = self.pos.saturating_add(len);
        if end > self.bytes.len() {
            return Err(anyhow!("unexpected EOF"));
        }
        let slice = &self.bytes[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    fn read_u8(&mut self) -> Result<u8> {
        Ok(self.read_exact(1)?[0])
    }

    fn read_u16(&mut self) -> Result<u16> {
        let bytes = self.read_exact(2)?;
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }

    fn read_u32(&mut self) -> Result<u32> {
        let bytes = self.read_exact(4)?;
        Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn read_u64(&mut self) -> Result<u64> {
        let bytes = self.read_exact(8)?;
        Ok(u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    fn read_varint_u64(&mut self) -> Result<u64> {
        let mut value = 0u64;
        let mut shift = 0u32;
        loop {
            let byte = self.read_u8()?;
            value |= ((byte & 0x7F) as u64) << shift;
            if (byte & 0x80) == 0 {
                break;
            }
            shift += 7;
            if shift >= 64 {
                return Err(anyhow!("varint too large"));
            }
        }
        Ok(value)
    }

    fn read_string(&mut self) -> Result<String> {
        let len = self.read_varint_u64()? as usize;
        let bytes = self.read_exact(len)?;
        let value = std::str::from_utf8(bytes).context("invalid utf8 string")?;
        Ok(value.to_string())
    }

    fn read_id(&mut self) -> Result<String> {
        let len = self.read_u32()? as usize;
        let bytes = self.read_exact(len)?;
        let value = std::str::from_utf8(bytes)
            .context("invalid utf8 id")?
            .to_string();
        validate_id(&value).map_err(|err| anyhow!("invalid id: {}", err))?;
        Ok(value)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = parse_options()?;
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(&database_url).await?;

    let meta = fetch_snapshot_meta(&pool, &options.selection).await?;
    let meta = meta.ok_or_else(|| anyhow!("snapshot not found"))?;

    let artifact_path = meta
        .artifact_path
        .clone()
        .ok_or_else(|| anyhow!("snapshot missing artifact_path"))?;
    let artifact_full_path = resolve_artifact_path(&artifact_path)?;
    let artifact_bytes = fs::read(&artifact_full_path)
        .with_context(|| format!("reading artifact {}", artifact_full_path.display()))?;

    let mut mismatches = Vec::new();

    let artifact_sha = sha256_hex(&artifact_bytes);
    if let Some(expected) = &meta.artifact_sha256 {
        if expected != &artifact_sha {
            mismatches.push(format!(
                "artifact_sha256 mismatch expected={} actual={}",
                expected, artifact_sha
            ));
        }
    } else {
        mismatches.push("artifact_sha256 missing in metadata".to_string());
    }

    let parsed = parse_snapshot_bytes(&artifact_bytes)?;

    if parsed.format_version != 0 {
        mismatches.push(format!(
            "format_version mismatch expected=0 actual={}",
            parsed.format_version
        ));
    }

    if parsed.header_flags != 0 {
        mismatches.push(format!(
            "header_flags mismatch expected=0 actual={}",
            parsed.header_flags
        ));
    }

    if parsed.snapshot_kind != 0 {
        mismatches.push(format!(
            "snapshot_kind mismatch expected=0 actual={}",
            parsed.snapshot_kind
        ));
    }

    if matches!(options.profile, Profile::Stage0) && !parsed.snapshot_tier_id.is_empty() {
        mismatches.push(format!(
            "profile stage0 expects empty snapshot_tier_id; got {}",
            parsed.snapshot_tier_id
        ));
    }

    if parsed.block_height != meta.block_height as u64 {
        mismatches.push(format!(
            "block_height mismatch expected={} actual={}",
            meta.block_height, parsed.block_height
        ));
    }

    if let Some(expected) = meta.event_count {
        if parsed.event_count != expected as u64 {
            mismatches.push(format!(
                "event_count mismatch expected={} actual={}",
                expected, parsed.event_count
            ));
        }
    }

    let parsed_last_event_id = Uuid::parse_str(&parsed.last_event_id)
        .map_err(|_| anyhow!("invalid last_event_id in snapshot header"))?;
    if let Some(expected) = meta.last_event_id {
        if parsed_last_event_id != expected {
            mismatches.push(format!(
                "last_event_id mismatch expected={} actual={}",
                expected, parsed_last_event_id
            ));
        }
    }

    let mut computed_section_hashes = BTreeMap::new();
    for section in &parsed.sections {
        let computed = section_hash(section.id, &section.bytes);
        computed_section_hashes.insert(section.id, computed.clone());
        if computed != section.hash {
            mismatches.push(format!(
                "section_hash mismatch id=0x{:04x} expected={} actual={}",
                section.id,
                to_hex(&section.hash),
                to_hex(&computed)
            ));
        }
        if section.bytes.len() as u64 != section.byte_len {
            mismatches.push(format!(
                "section_byte_len mismatch id=0x{:04x} expected={} actual={}",
                section.id,
                section.byte_len,
                section.bytes.len()
            ));
        }
        if section.item_count == 0 && !section.bytes.is_empty() {
            mismatches.push(format!(
                "section_item_count mismatch id=0x{:04x} item_count=0 but bytes present",
                section.id
            ));
        }
    }

    let computed_state_root = match options.profile {
        Profile::Stage0 => compute_stage0_state_root(&computed_section_hashes)?,
    };
    if computed_state_root != parsed.state_root_hash {
        mismatches.push(format!(
            "state_root_hash mismatch header={} computed={}",
            to_hex(&parsed.state_root_hash),
            to_hex(&computed_state_root)
        ));
    }
    if let Some(expected) = &meta.state_root_hash {
        if expected != &to_hex(&parsed.state_root_hash) {
            mismatches.push(format!(
                "state_root_hash metadata mismatch expected={} header={}",
                expected,
                to_hex(&parsed.state_root_hash)
            ));
        }
    }

    let replay_output = ReplayDriver::run(&pool, Some(meta.block_height))
        .await
        .map_err(|err| anyhow!("replay failed for payload root recompute: {}", err))?;
    let payload_root = compute_title_sentence_payload_root(&replay_output.payloads)
        .map_err(|err| anyhow!("payload_root compute failed: {}", err))?;
    if payload_root != parsed.title_sentence_payload_root {
        mismatches.push(format!(
            "title_sentence_payload_root mismatch header={} computed={}",
            to_hex(&parsed.title_sentence_payload_root),
            to_hex(&payload_root)
        ));
    }
    if let Some(expected) = &meta.title_sentence_payload_root {
        if expected != &to_hex(&parsed.title_sentence_payload_root) {
            mismatches.push(format!(
                "title_sentence_payload_root metadata mismatch expected={} header={}",
                expected,
                to_hex(&parsed.title_sentence_payload_root)
            ));
        }
    }

    let active_rulebook_set_hash = hash_with_domain("snapshot_rulebook_set", &[]);
    if active_rulebook_set_hash != parsed.active_rulebook_set_hash {
        mismatches.push(format!(
            "active_rulebook_set_hash mismatch header={} computed={}",
            to_hex(&parsed.active_rulebook_set_hash),
            to_hex(&active_rulebook_set_hash)
        ));
    }
    if let Some(expected) = &meta.active_rulebook_set_hash {
        if expected != &to_hex(&parsed.active_rulebook_set_hash) {
            mismatches.push(format!(
                "active_rulebook_set_hash metadata mismatch expected={} header={}",
                expected,
                to_hex(&parsed.active_rulebook_set_hash)
            ));
        }
    }

    let shared_map_commitment = hash_with_domain(
        "shared_map_commitment_v0",
        &[computed_state_root.as_slice(), payload_root.as_slice()].concat(),
    );
    if let Some(expected) = &meta.shared_map_commitment {
        if expected != &to_hex(&shared_map_commitment) {
            mismatches.push(format!(
                "shared_map_commitment mismatch expected={} computed={}",
                expected,
                to_hex(&shared_map_commitment)
            ));
        }
    }

    let snapshot_hash = hash_with_domain("snapshot", &artifact_bytes);
    let snapshot_hash_hex = to_hex(&snapshot_hash);
    if meta.snapshot_hash != snapshot_hash_hex {
        mismatches.push(format!(
            "snapshot_hash mismatch expected={} computed={}",
            meta.snapshot_hash, snapshot_hash_hex
        ));
    }

    let expected_path = format!(
        "backend/var/snapshots/v0/{}/{}.snapshot",
        meta.block_height, meta.snapshot_hash
    );
    if artifact_path != expected_path {
        mismatches.push(format!(
            "artifact_path mismatch expected={} actual={}",
            expected_path, artifact_path
        ));
    }

    let profile_label = match options.profile {
        Profile::Stage0 => "stage0",
    };

    if mismatches.is_empty() {
        println!(
            "PASS snapshot-verify profile={} height={} snapshot_hash={} path={}",
            profile_label, meta.block_height, meta.snapshot_hash, artifact_path
        );
        Ok(())
    } else {
        println!(
            "FAIL snapshot-verify profile={} height={} snapshot_hash={} path={}",
            profile_label, meta.block_height, meta.snapshot_hash, artifact_path
        );
        for mismatch in mismatches {
            println!("  - {}", mismatch);
        }
        Err(anyhow!("snapshot verification failed"))
    }
}

struct Options {
    selection: Selection,
    profile: Profile,
}

fn parse_options() -> Result<Options> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        return Err(anyhow!(
            "usage: snapshot-verify [--profile stage0] --latest | --height <n> | --snapshot-id <hex>"
        ));
    }

    let mut selection: Option<Selection> = None;
    let mut profile = Profile::Stage0;
    let mut idx = 0;
    while idx < args.len() {
        match args[idx].as_str() {
            "--profile" => {
                let value = args.get(idx + 1).ok_or_else(|| {
                    anyhow!("missing value for --profile (expected: stage0)")
                })?;
                match value.as_str() {
                    "stage0" => profile = Profile::Stage0,
                    other => {
                        return Err(anyhow!(
                            "unsupported profile '{}' (expected: stage0)",
                            other
                        ))
                    }
                }
                idx += 2;
            }
            "--latest" => {
                selection = Some(Selection::Latest);
                idx += 1;
            }
            "--height" => {
                let value = args
                    .get(idx + 1)
                    .ok_or_else(|| anyhow!("missing value for --height"))?;
                selection = Some(Selection::Height(
                    value.parse::<i64>().context("invalid height")?,
                ));
                idx += 2;
            }
            "--snapshot-id" => {
                let value = args
                    .get(idx + 1)
                    .ok_or_else(|| anyhow!("missing value for --snapshot-id"))?;
                selection = Some(Selection::SnapshotHash(value.to_string()));
                idx += 2;
            }
            other => {
                return Err(anyhow!(
                    "unexpected argument '{}' (usage: snapshot-verify [--profile stage0] --latest | --height <n> | --snapshot-id <hex>)",
                    other
                ))
            }
        }
    }

    let selection = selection.ok_or_else(|| {
        anyhow!("missing selection flag (expected one of: --latest, --height, --snapshot-id)")
    })?;

    Ok(Options { selection, profile })
}

async fn fetch_snapshot_meta(pool: &PgPool, selection: &Selection) -> Result<Option<SnapshotMeta>> {
    let query = r#"
        SELECT
          block_height,
          snapshot_hash,
          state_root_hash,
          title_sentence_payload_root,
          shared_map_commitment,
          active_rulebook_set_hash,
          artifact_path,
          artifact_sha256,
          last_event_id,
          event_count
        FROM snapshots
    "#;

    let meta = match selection {
        Selection::Latest => {
            sqlx::query_as::<_, SnapshotMeta>(&format!(
                "{query} ORDER BY block_height DESC LIMIT 1"
            ))
            .fetch_optional(pool)
            .await?
        }
        Selection::Height(height) => {
            sqlx::query_as::<_, SnapshotMeta>(&format!("{query} WHERE block_height = $1"))
                .bind(height)
                .fetch_optional(pool)
                .await?
        }
        Selection::SnapshotHash(snapshot_hash) => {
            sqlx::query_as::<_, SnapshotMeta>(&format!("{query} WHERE snapshot_hash = $1"))
                .bind(snapshot_hash)
                .fetch_optional(pool)
                .await?
        }
    };

    Ok(meta)
}

fn parse_snapshot_bytes(bytes: &[u8]) -> Result<ParsedSnapshot> {
    let mut cursor = Cursor::new(bytes);
    let magic = cursor.read_exact(8)?;
    if magic != b"MCCSNAP0" {
        return Err(anyhow!("invalid magic header"));
    }

    let format_version = cursor.read_u16()?;
    let header_flags = cursor.read_u16()?;
    let header_len = cursor.read_u32()? as usize;
    let header_body = cursor.read_exact(header_len)?;

    let mut header_cursor = Cursor::new(header_body);
    let block_height = header_cursor.read_u64()?;
    let snapshot_kind = header_cursor.read_u8()?;
    let snapshot_tier_id = header_cursor.read_string()?;
    let last_event_id = header_cursor.read_id()?;
    let event_count = header_cursor.read_u64()?;
    let active_rulebook_set_hash = header_cursor.read_exact(32)?.to_vec();
    let state_root_hash = header_cursor.read_exact(32)?.to_vec();
    let title_sentence_payload_root = header_cursor.read_exact(32)?.to_vec();
    let section_count = header_cursor.read_u16()? as usize;

    let mut directory = Vec::with_capacity(section_count);
    for _ in 0..section_count {
        let id = header_cursor.read_u16()?;
        let item_count = header_cursor.read_u32()?;
        let byte_len = header_cursor.read_u64()?;
        let hash = header_cursor.read_exact(32)?.to_vec();
        directory.push((id, item_count, byte_len, hash));
    }

    if header_cursor.pos != header_body.len() {
        return Err(anyhow!(
            "header length mismatch expected={} actual={}",
            header_body.len(),
            header_cursor.pos
        ));
    }

    let mut sections = Vec::with_capacity(section_count);
    let mut body_pos = cursor.pos;
    for (id, item_count, byte_len, hash) in directory {
        let len = usize::try_from(byte_len).context("section length overflow")?;
        let end = body_pos
            .checked_add(len)
            .ok_or_else(|| anyhow!("section length overflow"))?;
        if end > bytes.len() {
            return Err(anyhow!("section length exceeds snapshot bytes"));
        }
        let section_bytes = bytes[body_pos..end].to_vec();
        body_pos = end;
        sections.push(ParsedSection {
            id,
            item_count,
            byte_len,
            hash,
            bytes: section_bytes,
        });
    }

    if body_pos != bytes.len() {
        return Err(anyhow!(
            "body length mismatch expected_end={} actual_end={}",
            bytes.len(),
            body_pos
        ));
    }

    Ok(ParsedSnapshot {
        format_version,
        header_flags,
        block_height,
        snapshot_kind,
        snapshot_tier_id,
        last_event_id,
        event_count,
        active_rulebook_set_hash,
        state_root_hash,
        title_sentence_payload_root,
        sections,
    })
}

fn section_hash(section_id: u16, section_bytes: &[u8]) -> Vec<u8> {
    let mut payload = Vec::with_capacity(2 + section_bytes.len());
    payload.extend_from_slice(&encode_u16(section_id));
    payload.extend_from_slice(section_bytes);
    hash_with_domain("snapshot_section", &payload)
}

fn compute_stage0_state_root(hashes: &BTreeMap<u16, Vec<u8>>) -> Result<Vec<u8>> {
    let ideas = hashes
        .get(&IDEAS_SECTION_ID)
        .ok_or_else(|| anyhow!("missing ideas section"))?;
    let connections = hashes
        .get(&CONNECTIONS_SECTION_ID)
        .ok_or_else(|| anyhow!("missing connections section"))?;
    let rails = hashes
        .get(&RAILS_SECTION_ID)
        .ok_or_else(|| anyhow!("missing rails section"))?;
    let mut payload = Vec::new();
    payload.extend_from_slice(ideas);
    payload.extend_from_slice(connections);
    payload.extend_from_slice(rails);
    Ok(hash_with_domain("snapshot_state_root", &payload))
}

fn resolve_artifact_path(artifact_path: &str) -> Result<PathBuf> {
    let candidate = Path::new(artifact_path);
    if candidate.is_absolute() {
        return Err(anyhow!(
            "artifact_path must be relative for portability; got absolute path"
        ));
    }
    if candidate
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(anyhow!(
            "artifact_path must not contain parent directory components"
        ));
    }

    let backend_root = backend_root()?;
    let repo_root = backend_root
        .parent()
        .ok_or_else(|| anyhow!("unable to resolve repo root"))?;

    let mut normalized = artifact_path.replace('/', &std::path::MAIN_SEPARATOR.to_string());
    if normalized.starts_with(std::path::MAIN_SEPARATOR) {
        normalized = normalized
            .trim_start_matches(std::path::MAIN_SEPARATOR)
            .to_string();
    }

    let rel = Path::new(&normalized);
    let mut use_repo_root = false;
    if let Some(Component::Normal(component)) = rel.components().next() {
        if component == "backend" {
            use_repo_root = true;
        }
    }
    let base = if use_repo_root {
        repo_root
    } else {
        &backend_root
    };
    Ok(base.join(rel))
}

fn backend_root() -> Result<PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let backend_root = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .ok_or_else(|| anyhow!("unable to resolve backend root"))?;
    Ok(backend_root.to_path_buf())
}
