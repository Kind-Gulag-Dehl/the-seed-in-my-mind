use unicode_normalization::UnicodeNormalization;

pub fn encode_u8(value: u8) -> Vec<u8> {
    vec![value]
}

pub fn encode_u16(value: u16) -> Vec<u8> {
    value.to_be_bytes().to_vec()
}

pub fn encode_u32(value: u32) -> Vec<u8> {
    value.to_be_bytes().to_vec()
}

pub fn encode_u64(value: u64) -> Vec<u8> {
    value.to_be_bytes().to_vec()
}

pub fn encode_varint_u64(mut value: u64) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
    out
}

pub fn encode_string(value: &str) -> Vec<u8> {
    let bytes = value.as_bytes();
    let mut out = encode_varint_u64(bytes.len() as u64);
    out.extend_from_slice(bytes);
    out
}

pub fn validate_id(value: &str) -> Result<(), String> {
    if value.len() != 36 {
        return Err("id must be 36 characters".to_string());
    }
    let bytes = value.as_bytes();
    for (idx, ch) in bytes.iter().enumerate() {
        let is_hyphen = matches!(idx, 8 | 13 | 18 | 23);
        if is_hyphen {
            if *ch != b'-' {
                return Err("id must use hyphens at 8-4-4-4-12 positions".to_string());
            }
            continue;
        }
        if !matches!(ch, b'0'..=b'9' | b'a'..=b'f') {
            return Err("id must use lowercase hex".to_string());
        }
    }
    if bytes[14] != b'7' {
        return Err("id must be UUIDv7".to_string());
    }
    if !matches!(bytes[19], b'8' | b'9' | b'a' | b'b') {
        return Err("id must have RFC 4122 variant".to_string());
    }
    Ok(())
}

pub fn encode_id(value: &str) -> Result<Vec<u8>, String> {
    validate_id(value)?;
    let mut out = encode_u32(36);
    out.extend_from_slice(value.as_bytes());
    Ok(out)
}

pub fn canonicalize_utf8(input: &[u8]) -> Result<Vec<u8>, String> {
    if input.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Err("BOM is not allowed".to_string());
    }
    let text = std::str::from_utf8(input).map_err(|_| "invalid UTF-8".to_string())?;
    let normalized = text.nfc().collect::<String>();

    let mut out = String::with_capacity(normalized.len());
    let mut chars = normalized.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\r' {
            if matches!(chars.peek(), Some('\n')) {
                chars.next();
            }
            out.push('\n');
        } else {
            out.push(ch);
        }
    }
    if out.contains('\r') {
        return Err("carriage return not allowed after normalization".to_string());
    }
    Ok(out.into_bytes())
}

pub fn canonicalize_string(value: &str) -> Result<Vec<u8>, String> {
    canonicalize_utf8(value.as_bytes())
}
