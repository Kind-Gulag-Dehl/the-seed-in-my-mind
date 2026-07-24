const APPROVED_EXACT_TEST_DATABASES: &[&str] = &["seed_export_smoke"];
const APPROVED_TEST_DATABASE_PREFIXES: &[&str] = &[
    "seed_admission_p3_test_",
    "seed_signed_ingress_test_",
    "seed_tempo_005c_r2_",
    "seed_test_",
];
const PROTECTED_DATABASES: &[&str] = &[
    "seed_dev",
    "seed_open_core",
    "postgres",
    "template0",
    "template1",
];

pub fn require_disposable_database_url(database_url: &str) -> std::result::Result<String, String> {
    let database_name = database_name_from_url(database_url)?;
    let normalized = database_name.to_ascii_lowercase();

    if PROTECTED_DATABASES.contains(&normalized.as_str()) {
        return Err(format!(
            "database target `{}` is protected or maintenance state; use an explicit disposable test database",
            normalized
        ));
    }

    if APPROVED_EXACT_TEST_DATABASES.contains(&normalized.as_str())
        || APPROVED_TEST_DATABASE_PREFIXES
            .iter()
            .any(|prefix| normalized.starts_with(prefix))
    {
        return Ok(database_name);
    }

    Err(format!(
        "database target `{}` is not an approved disposable test database",
        normalized
    ))
}

fn database_name_from_url(database_url: &str) -> std::result::Result<String, String> {
    let trimmed = database_url.trim();
    if trimmed.is_empty() {
        return Err("database URL is empty".to_string());
    }

    let without_query = trimmed.split(['?', '#']).next().unwrap_or(trimmed);
    let Some((scheme, rest)) = without_query.split_once("://") else {
        return Err("database URL must include a scheme and database path".to_string());
    };
    if !matches!(scheme, "postgres" | "postgresql") {
        return Err("database URL scheme must be postgres or postgresql".to_string());
    }

    let Some((_, path)) = rest.split_once('/') else {
        return Err("database URL is missing a database name".to_string());
    };
    let database_name = path.trim_matches('/');
    if database_name.is_empty() || database_name.contains('/') {
        return Err("database URL is missing a single database name".to_string());
    }
    Ok(database_name.to_string())
}

#[cfg(test)]
mod tests {
    use super::require_disposable_database_url;

    #[test]
    fn accepts_approved_disposable_names() {
        assert_eq!(
            require_disposable_database_url(
                "postgresql://seed_app:secret@127.0.0.1:5432/seed_tempo_005c_r2_cmd_abcd"
            )
            .unwrap(),
            "seed_tempo_005c_r2_cmd_abcd"
        );
        assert_eq!(
            require_disposable_database_url(
                "postgresql://seed_app:secret@127.0.0.1:5432/seed_export_smoke"
            )
            .unwrap(),
            "seed_export_smoke"
        );
        assert_eq!(
            require_disposable_database_url(
                "postgresql://seed_app:secret@127.0.0.1:5432/seed_admission_p3_test_019f"
            )
            .unwrap(),
            "seed_admission_p3_test_019f"
        );
    }

    #[test]
    fn permits_signed_ingress_database_names() {
        assert_eq!(
            require_disposable_database_url(
                "postgresql://seed_app@127.0.0.1:5432/seed_signed_ingress_test_123_019f"
            )
            .unwrap(),
            "seed_signed_ingress_test_123_019f"
        );
    }

    #[test]
    fn rejects_protected_and_ordinary_targets_without_printing_credentials() {
        let err = require_disposable_database_url(
            "postgresql://seed_app:super-secret-password@127.0.0.1:5432/seed_dev",
        )
        .expect_err("seed_dev must be rejected");
        assert!(err.contains("seed_dev"));
        assert!(!err.contains("super-secret-password"));

        let err = require_disposable_database_url(
            "postgresql://seed_app:super-secret-password@127.0.0.1:5432/seed_open_core",
        )
        .expect_err("ordinary app database must be rejected");
        assert!(err.contains("seed_open_core"));
        assert!(!err.contains("super-secret-password"));
    }

    #[test]
    fn rejects_absent_or_admin_database_targets() {
        assert!(require_disposable_database_url("postgresql://seed_app@127.0.0.1:5432").is_err());
        let err = require_disposable_database_url("postgresql://seed_app@127.0.0.1:5432/postgres")
            .expect_err("maintenance database must be rejected");
        assert!(err.contains("postgres"));
    }
}
