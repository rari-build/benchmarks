use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::path::Path;
use tokio::fs;

pub async fn read_latest(results_dir: &Path) -> Result<Value> {
    let latest_path = results_dir.join("latest.json");
    if !latest_path.exists() {
        return Ok(json!({}));
    }

    let existing = fs::read_to_string(&latest_path).await?;
    Ok(serde_json::from_str::<Value>(&existing).unwrap_or_else(|_| json!({})))
}

pub async fn write_latest(results_dir: &Path, root: &Value) -> Result<()> {
    fs::create_dir_all(results_dir).await?;
    let json = format!("{}\n", serde_json::to_string_pretty(root)?);
    fs::write(results_dir.join("latest.json"), json)
        .await
        .context("Failed to write latest.json")?;
    Ok(())
}
