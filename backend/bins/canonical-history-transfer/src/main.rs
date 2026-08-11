use anyhow::{anyhow, Context, Result};
use canonical_history::{
    database_name_from_url, export_database, import_database, validate_package, ImportOutcome,
    ResourceLimits,
};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
enum Command {
    Export {
        database_url: String,
        output: PathBuf,
    },
    Import {
        database_url: Option<String>,
        package: PathBuf,
        confirm_fresh_target: Option<String>,
        validate_only: bool,
    },
    Validate {
        package: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    match parse_args()? {
        Command::Export {
            database_url,
            output,
        } => {
            let pool = PgPoolOptions::new()
                .max_connections(2)
                .connect(&database_url)
                .await
                .context("connect source database")?;
            let manifest = export_database(&pool, &output, ResourceLimits::default()).await?;
            println!(
                "canonical-history-transfer: export pass height={} events={} package_hash={} output={}",
                manifest.source.height,
                manifest.source.event_count,
                manifest.whole_package_hash,
                output.display()
            );
        }
        Command::Validate { package } => {
            let validated = validate_package(&package, ResourceLimits::default())?;
            println!(
                "canonical-history-transfer: validate-only pass database_writes=0 height={} events={} package_hash={}",
                validated.manifest.source.height,
                validated.events.len(),
                validated.manifest.whole_package_hash
            );
        }
        Command::Import {
            database_url,
            package,
            confirm_fresh_target,
            validate_only,
        } => {
            let validated = validate_package(&package, ResourceLimits::default())?;
            if validate_only {
                println!(
                    "canonical-history-transfer: validate-only pass database_writes=0 height={} events={} package_hash={}",
                    validated.manifest.source.height,
                    validated.events.len(),
                    validated.manifest.whole_package_hash
                );
                return Ok(());
            }
            let database_url = database_url
                .or_else(|| env::var("DATABASE_URL").ok())
                .ok_or_else(|| anyhow!("--database-url or DATABASE_URL is required for import"))?;
            let actual_name = database_name_from_url(&database_url)?;
            let confirmed = confirm_fresh_target
                .ok_or_else(|| anyhow!("--confirm-fresh-target <database-name> is required"))?;
            if confirmed != actual_name {
                return Err(anyhow!(
                    "--confirm-fresh-target mismatch: expected exact target database name"
                ));
            }
            let pool = PgPoolOptions::new()
                .max_connections(2)
                .connect(&database_url)
                .await
                .context("connect target database")?;
            let report = import_database(&pool, &validated).await?;
            let outcome = match report.outcome {
                ImportOutcome::Imported => "imported",
                ImportOutcome::AlreadyPresent => "already_present",
            };
            println!(
                "canonical-history-transfer: import pass outcome={} height={} events={} state_root_hash={} payload_root={} shared_map_commitment={} snapshot_hash={}",
                outcome,
                report.height,
                report.event_count,
                report.state_root_hash,
                report.title_sentence_payload_root,
                report.shared_map_commitment,
                report.snapshot_hash
            );
        }
    }
    Ok(())
}

fn parse_args() -> Result<Command> {
    let mut args = env::args().skip(1);
    let subcommand = args.next().ok_or_else(|| anyhow!(usage()))?;
    let mut database_url = None;
    let mut output = None;
    let mut package = None;
    let mut confirm_fresh_target = None;
    let mut validate_only = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--database-url" => {
                database_url = Some(
                    args.next()
                        .ok_or_else(|| anyhow!("missing --database-url value"))?,
                );
            }
            "--output" => {
                output = Some(PathBuf::from(
                    args.next()
                        .ok_or_else(|| anyhow!("missing --output value"))?,
                ));
            }
            "--package" => {
                package = Some(PathBuf::from(
                    args.next()
                        .ok_or_else(|| anyhow!("missing --package value"))?,
                ));
            }
            "--confirm-fresh-target" => {
                confirm_fresh_target = Some(
                    args.next()
                        .ok_or_else(|| anyhow!("missing --confirm-fresh-target value"))?,
                );
            }
            "--validate-only" => validate_only = true,
            _ => return Err(anyhow!("unexpected argument {arg}\n{}", usage())),
        }
    }

    match subcommand.as_str() {
        "export" => Ok(Command::Export {
            database_url: database_url
                .or_else(|| env::var("DATABASE_URL").ok())
                .ok_or_else(|| anyhow!("--database-url or DATABASE_URL is required for export"))?,
            output: output.ok_or_else(|| anyhow!("--output is required for export"))?,
        }),
        "import" => Ok(Command::Import {
            database_url,
            package: package.ok_or_else(|| anyhow!("--package is required for import"))?,
            confirm_fresh_target,
            validate_only,
        }),
        "validate" => Ok(Command::Validate {
            package: package.ok_or_else(|| anyhow!("--package is required for validate"))?,
        }),
        _ => Err(anyhow!(usage())),
    }
}

fn usage() -> String {
    "usage: canonical-history-transfer export --output <dir> [--database-url <url>] | import --package <dir> [--validate-only] [--database-url <url> --confirm-fresh-target <name>] | validate --package <dir>".to_string()
}
