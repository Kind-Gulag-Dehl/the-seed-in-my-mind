use crate::canonical::{canonicalize_string, encode_u8, encode_varint_u64};
use crate::hash::hash_bytes;
use serde_json::{Map, Value};

fn encode_string_canon(value: &str) -> Result<Vec<u8>, String> {
    let bytes = canonicalize_string(value)?;
    let mut out = encode_varint_u64(bytes.len() as u64);
    out.extend_from_slice(&bytes);
    Ok(out)
}

fn push_optional_string(out: &mut Vec<u8>, value: Option<&str>) -> Result<(), String> {
    match value {
        Some(value) => {
            out.extend_from_slice(&encode_u8(1));
            out.extend_from_slice(&encode_string_canon(value)?);
        }
        None => {
            out.extend_from_slice(&encode_u8(0));
        }
    }
    Ok(())
}

fn payload_bytes(
    title: &str,
    sentence: &str,
    paragraph: Option<&str>,
    full: Option<&str>,
) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    push_optional_string(&mut out, Some(title))?;
    push_optional_string(&mut out, Some(sentence))?;
    push_optional_string(&mut out, paragraph)?;
    push_optional_string(&mut out, full)?;
    Ok(out)
}

pub fn payload_hash_hex(
    title: &str,
    sentence: &str,
    paragraph: Option<&str>,
    full: Option<&str>,
) -> Result<String, String> {
    let bytes = payload_bytes(title, sentence, paragraph, full)?;
    let hash = hash_bytes(&bytes);
    Ok(to_hex(&hash))
}

pub fn canonical_json_payload_bytes(value: &Value) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    write_canonical_json(value, &mut out)?;
    Ok(out)
}

pub fn canonical_json_payload_hash_hex(value: &Value) -> Result<String, String> {
    let bytes = canonical_json_payload_bytes(value)?;
    Ok(to_hex(&hash_bytes(&bytes)))
}

fn write_canonical_json(value: &Value, out: &mut Vec<u8>) -> Result<(), String> {
    match value {
        Value::Null => out.extend_from_slice(b"null"),
        Value::Bool(true) => out.extend_from_slice(b"true"),
        Value::Bool(false) => out.extend_from_slice(b"false"),
        Value::Number(number) => {
            if let Some(value) = number.as_i64() {
                out.extend_from_slice(value.to_string().as_bytes());
            } else if let Some(value) = number.as_u64() {
                out.extend_from_slice(value.to_string().as_bytes());
            } else {
                return Err(
                    "floating-point JSON numbers are not supported in canonical payloads"
                        .to_string(),
                );
            }
        }
        Value::String(value) => {
            let normalized = normalized_string(value)?;
            out.extend_from_slice(
                serde_json::to_string(&normalized)
                    .map_err(|err| err.to_string())?
                    .as_bytes(),
            );
        }
        Value::Array(values) => {
            out.push(b'[');
            for (idx, item) in values.iter().enumerate() {
                if idx > 0 {
                    out.push(b',');
                }
                write_canonical_json(item, out)?;
            }
            out.push(b']');
        }
        Value::Object(map) => write_canonical_json_object(map, out)?,
    }
    Ok(())
}

fn write_canonical_json_object(map: &Map<String, Value>, out: &mut Vec<u8>) -> Result<(), String> {
    let mut entries = Vec::with_capacity(map.len());
    for (key, value) in map {
        let normalized = normalized_string(key)?;
        entries.push((normalized, value));
    }
    entries.sort_by(|left, right| left.0.cmp(&right.0));

    out.push(b'{');
    for (idx, (key, value)) in entries.iter().enumerate() {
        if idx > 0 {
            out.push(b',');
        }
        out.extend_from_slice(
            serde_json::to_string(key)
                .map_err(|err| err.to_string())?
                .as_bytes(),
        );
        out.push(b':');
        write_canonical_json(value, out)?;
    }
    out.push(b'}');
    Ok(())
}

fn normalized_string(value: &str) -> Result<String, String> {
    let bytes = canonicalize_string(value)?;
    String::from_utf8(bytes).map_err(|_| "invalid UTF-8 after normalization".to_string())
}

pub fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}
