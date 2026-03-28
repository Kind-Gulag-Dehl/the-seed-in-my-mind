use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecretPattern {
    PemPrivateKey,
    BearerToken,
    JwtLikeToken,
    DsnWithCredentials,
    SensitiveAssignment,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretScreenHit {
    pub pattern: SecretPattern,
    pub path: String,
}

pub fn screen_text_for_secrets(text: &str) -> Option<SecretPattern> {
    if text.trim().is_empty() {
        return None;
    }
    if contains_pem_private_key(text) {
        return Some(SecretPattern::PemPrivateKey);
    }
    if contains_bearer_token(text) {
        return Some(SecretPattern::BearerToken);
    }
    if contains_jwt_like_token(text) {
        return Some(SecretPattern::JwtLikeToken);
    }
    if contains_dsn_with_credentials(text) {
        return Some(SecretPattern::DsnWithCredentials);
    }
    if contains_sensitive_assignment(text) {
        return Some(SecretPattern::SensitiveAssignment);
    }
    None
}

pub fn screen_json_for_secrets(value: &Value) -> Option<SecretScreenHit> {
    screen_json_inner(value, "$")
}

fn screen_json_inner(value: &Value, path: &str) -> Option<SecretScreenHit> {
    match value {
        Value::String(text) => screen_text_for_secrets(text).map(|pattern| SecretScreenHit {
            pattern,
            path: path.to_string(),
        }),
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                let child = format!("{path}[{index}]");
                if let Some(hit) = screen_json_inner(item, child.as_str()) {
                    return Some(hit);
                }
            }
            None
        }
        Value::Object(object) => {
            for (key, item) in object {
                let child = format!("{path}.{key}");
                if let Some(hit) = screen_json_inner(item, child.as_str()) {
                    return Some(hit);
                }
            }
            None
        }
        _ => None,
    }
}

fn contains_pem_private_key(text: &str) -> bool {
    let upper = text.to_ascii_uppercase();
    upper.contains("-----BEGIN PRIVATE KEY-----")
        || upper.contains("-----BEGIN RSA PRIVATE KEY-----")
        || upper.contains("-----BEGIN EC PRIVATE KEY-----")
        || upper.contains("-----BEGIN OPENSSH PRIVATE KEY-----")
}

fn contains_bearer_token(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    const MARKERS: [&str; 2] = ["authorization: bearer ", "bearer "];
    for marker in MARKERS {
        let mut offset = 0usize;
        while let Some(found) = lower[offset..].find(marker) {
            let token_start = offset + found + marker.len();
            let token_len = text[token_start..]
                .chars()
                .take_while(|value| is_token_char(*value))
                .count();
            if token_len >= 20 {
                return true;
            }
            offset = token_start;
        }
    }
    false
}

fn contains_jwt_like_token(text: &str) -> bool {
    for candidate in text.split(|value: char| {
        value.is_whitespace()
            || matches!(
                value,
                '"' | '\'' | ';' | ',' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>'
            )
    }) {
        let token = candidate.trim();
        if looks_like_jwt(token) {
            return true;
        }
    }
    false
}

fn looks_like_jwt(value: &str) -> bool {
    let mut parts = value.split('.');
    let Some(first) = parts.next() else {
        return false;
    };
    let Some(second) = parts.next() else {
        return false;
    };
    let Some(third) = parts.next() else {
        return false;
    };
    if parts.next().is_some() {
        return false;
    }

    if first.len() < 8 || second.len() < 8 || third.len() < 8 {
        return false;
    }
    if !first.starts_with("eyJ") {
        return false;
    }

    is_base64urlish(first) && is_base64urlish(second) && is_base64urlish(third)
}

fn contains_dsn_with_credentials(text: &str) -> bool {
    let mut offset = 0usize;
    while let Some(found) = text[offset..].find("://") {
        let scheme_idx = offset + found;
        let authority_start = scheme_idx + 3;
        let Some(authority_end_relative) = text[authority_start..]
            .chars()
            .position(|value| value.is_whitespace() || matches!(value, '/' | '?' | '#'))
        else {
            return authority_has_credentials(&text[authority_start..]);
        };
        let authority_end = authority_start + authority_end_relative;
        if authority_has_credentials(&text[authority_start..authority_end]) {
            return true;
        }
        offset = authority_start;
    }
    false
}

fn contains_sensitive_assignment(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    const KEYS: [&str; 5] = ["password", "apikey", "api_key", "secret", "token"];
    for key in KEYS {
        if contains_key_assignment(lower.as_str(), key) {
            return true;
        }
    }
    false
}

fn contains_key_assignment(text: &str, key: &str) -> bool {
    let mut offset = 0usize;
    while let Some(found) = text[offset..].find(key) {
        let key_start = offset + found;
        let key_end = key_start + key.len();
        if !is_word_boundary(text, key_start, key_end) {
            offset = key_end;
            continue;
        }

        let mut delimiter_index = key_end;
        while let Some(next) = text.as_bytes().get(delimiter_index) {
            if next.is_ascii_whitespace() {
                delimiter_index += 1;
                continue;
            }
            if *next == b'=' || *next == b':' {
                delimiter_index += 1;
                while let Some(value) = text.as_bytes().get(delimiter_index) {
                    if value.is_ascii_whitespace() {
                        delimiter_index += 1;
                        continue;
                    }
                    let value_start = delimiter_index;
                    let mut value_end = delimiter_index;
                    while let Some(ch) = text.as_bytes().get(value_end) {
                        if is_assignment_separator(*ch) {
                            break;
                        }
                        value_end += 1;
                    }
                    let value = text[value_start..value_end]
                        .trim_matches(|ch| matches!(ch, '"' | '\'' | '`'));
                    return is_secret_like_value(value);
                }
            }
            break;
        }
        offset = key_end;
    }
    false
}

fn authority_has_credentials(authority: &str) -> bool {
    let Some(password) = authority_password(authority) else {
        return false;
    };
    is_secret_like_value(password)
}

fn authority_password(authority: &str) -> Option<&str> {
    let at_index = authority.find('@')?;
    let user_info = &authority[..at_index];
    let colon_index = user_info.find(':')?;
    if colon_index == 0 || colon_index + 1 >= user_info.len() {
        return None;
    }
    Some(&user_info[colon_index + 1..])
}

fn is_assignment_separator(value: u8) -> bool {
    value.is_ascii_whitespace() || matches!(value, b',' | b';' | b'}' | b']')
}

fn is_secret_like_value(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    if is_obvious_placeholder(trimmed) {
        return false;
    }
    trimmed.len() >= 6
}

fn is_obvious_placeholder(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    if lower.starts_with('<') && lower.ends_with('>') {
        return true;
    }
    if lower.starts_with('{') && lower.ends_with('}') {
        return true;
    }
    if lower.starts_with('[') && lower.ends_with(']') {
        return true;
    }
    if lower.trim_matches('*').is_empty() {
        return true;
    }
    if matches!(
        lower.as_str(),
        "password"
            | "token"
            | "secret"
            | "apikey"
            | "api_key"
            | "your_password"
            | "your-password"
            | "yourpassword"
            | "password_here"
            | "token_here"
            | "secret_here"
            | "changeme"
            | "redacted"
            | "placeholder"
            | "dummy"
            | "sample"
            | "example"
    ) {
        return true;
    }
    lower.contains("example") || lower.contains("placeholder") || lower.contains("redacted")
}

fn is_token_char(value: char) -> bool {
    value.is_ascii_alphanumeric() || matches!(value, '-' | '_' | '.' | '~' | '+' | '/' | '=' | ':')
}

fn is_base64urlish(value: &str) -> bool {
    value
        .chars()
        .all(|item| item.is_ascii_alphanumeric() || matches!(item, '_' | '-' | '='))
}

fn is_word_boundary(text: &str, start: usize, end: usize) -> bool {
    let bytes = text.as_bytes();
    let before = start
        .checked_sub(1)
        .and_then(|index| bytes.get(index))
        .copied();
    let after = bytes.get(end).copied();
    let before_ok = before
        .map(|value| !value.is_ascii_alphanumeric() && value != b'_')
        .unwrap_or(true);
    let after_ok = after
        .map(|value| !value.is_ascii_alphanumeric() && value != b'_')
        .unwrap_or(true);
    before_ok && after_ok
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn detects_pem_private_key() {
        let hit = screen_text_for_secrets("-----BEGIN RSA PRIVATE KEY-----\nabc");
        assert_eq!(hit, Some(SecretPattern::PemPrivateKey));
    }

    #[test]
    fn detects_bearer_token() {
        let text = "Authorization: Bearer abcdefghijklmnopqrstuvwxyz0123456789";
        let hit = screen_text_for_secrets(text);
        assert_eq!(hit, Some(SecretPattern::BearerToken));
    }

    #[test]
    fn detects_jwt_like_token() {
        let text = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.c2lnbmF0dXJlX3NhbXBsZQ";
        let hit = screen_text_for_secrets(text);
        assert_eq!(hit, Some(SecretPattern::JwtLikeToken));
    }

    #[test]
    fn detects_dsn_with_credentials() {
        let text = "postgres://seed_user:s3ed-pass-123@localhost:5432/seeddb";
        let hit = screen_text_for_secrets(text);
        assert_eq!(hit, Some(SecretPattern::DsnWithCredentials));
    }

    #[test]
    fn detects_sensitive_assignments() {
        for value in [
            "password=abc123",
            "apikey: abc123",
            "api_key = abc123",
            "secret = abc123",
            "token: abc123",
        ] {
            let hit = screen_text_for_secrets(value);
            assert_eq!(hit, Some(SecretPattern::SensitiveAssignment));
        }
    }

    #[test]
    fn detects_nested_json_secret() {
        let payload = json!({
            "root": {
                "notes": "safe",
                "nested": ["safe", "Bearer abcdefghijklmnopqrstuvwxyz0123456789"]
            }
        });
        let hit = screen_json_for_secrets(&payload).expect("secret hit");
        assert_eq!(hit.pattern, SecretPattern::BearerToken);
        assert_eq!(hit.path, "$.root.nested[1]");
    }

    #[test]
    fn ignores_safe_text() {
        let hit = screen_text_for_secrets("focus on canonical ordering and replay determinism");
        assert_eq!(hit, None);
    }

    #[test]
    fn ignores_placeholder_assignment_examples() {
        let hit = screen_text_for_secrets("Example docs: password=<YOUR_PASSWORD>");
        assert_eq!(hit, None);

        let hit = screen_text_for_secrets("Use token=placeholder in local tutorial snippets.");
        assert_eq!(hit, None);
    }

    #[test]
    fn ignores_sentence_that_mentions_assignment_syntax() {
        let hit = screen_text_for_secrets(
            "The parser should accept the literal pattern password= in prose.",
        );
        assert_eq!(hit, None);
    }

    #[test]
    fn ignores_placeholder_dsn_examples() {
        let hit = screen_text_for_secrets("postgres://user:password@example.com/mydb");
        assert_eq!(hit, None);
    }
}
