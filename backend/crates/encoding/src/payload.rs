use crate::canonical::{canonicalize_string, encode_u8, encode_varint_u64};
use crate::hash::hash_bytes;

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

fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}
