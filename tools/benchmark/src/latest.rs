use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::path::Path;
use tokio::fs;

/// Loads `latest.json` from `results_dir`, or an empty object if it is missing.
///
/// # Errors
///
/// Returns an error if the file exists but cannot be read.
pub async fn read_latest(results_dir: &Path) -> Result<Value> {
    let latest_path = results_dir.join("latest.json");
    if !latest_path.exists() {
        return Ok(json!({}));
    }

    let existing = fs::read_to_string(&latest_path)
        .await
        .with_context(|| format!("Failed to read {}", latest_path.display()))?;
    Ok(serde_json::from_str::<Value>(&existing).unwrap_or_else(|_| json!({})))
}

/// Writes `root` to `results_dir/latest.json`, creating the directory if needed.
///
/// # Errors
///
/// Returns an error if the directory cannot be created, JSON cannot be serialized,
/// or the file cannot be written.
pub async fn write_latest(results_dir: &Path, root: &Value) -> Result<()> {
    fs::create_dir_all(results_dir)
        .await
        .with_context(|| format!("Failed to create {}", results_dir.display()))?;
    let json = format!("{}\n", serde_json::to_string_pretty(root)?);
    fs::write(results_dir.join("latest.json"), json)
        .await
        .context("Failed to write latest.json")?;
    Ok(())
}
