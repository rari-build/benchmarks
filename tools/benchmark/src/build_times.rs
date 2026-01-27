mod time_utils;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::time::{Instant, SystemTime};
use tokio::fs;
use tokio::process::Command;

#[derive(Parser, Debug)]
#[command(name = "build-times")]
#[command(about = "Compare build times between rari and Next.js")]
struct Args {
    #[arg(short, long, default_value = ".")]
    dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct BuildResult {
    success: bool,
    duration_ms: f64,
    bundle_size: Option<String>,
    chunk_count: Option<usize>,
    warnings: usize,
    errors: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkResults {
    timestamp: String,
    rari: BuildResult,
    nextjs: BuildResult,
}

async fn run_build(name: &str, directory: &Path, command: &str) -> Result<BuildResult> {
    println!("\n{} Building {}...", "🔨".bold(), name.bold());
    println!("  {} {}", "Directory:".dimmed(), directory.display());
    println!("  {} {}", "Command:".dimmed(), command);

    let start = Instant::now();

    let parts: Vec<&str> = command.split_whitespace().collect();
    let (cmd, args) = parts.split_first().context("Empty command")?;

    let output = Command::new(cmd)
        .args(args)
        .current_dir(directory)
        .env("NODE_ENV", "production")
        .output()
        .await
        .context("Failed to execute build command")?;

    let duration = start.elapsed();
    let duration_ms = duration.as_secs_f64() * 1000.0;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);

    let success = output.status.success();

    if success {
        println!(
            "  {} {} built successfully in {:.2}ms",
            "✅".green(),
            name,
            duration_ms
        );
    } else {
        println!(
            "  {} {} build failed (exit code {})",
            "❌".red(),
            name,
            output.status.code().unwrap_or(-1)
        );
        if !stderr.is_empty() {
            println!("  {} {}", "Error:".red(), stderr.trim());
        }
    }

    let deprecation_warnings =
        combined.matches("DeprecationWarning").count() + combined.matches("[DEP").count();

    let warnings = (combined.matches("warning").count()
        + combined.matches("Warning").count()
        + combined.matches("WARNING").count())
    .saturating_sub(deprecation_warnings);

    let errors = combined.matches("error").count()
        + combined.matches("Error").count()
        + combined.matches("ERROR").count();

    let (bundle_size, chunk_count) = if success {
        get_bundle_info(name, directory).await?
    } else {
        (None, None)
    };

    Ok(BuildResult {
        success,
        duration_ms,
        bundle_size,
        chunk_count,
        warnings,
        errors,
    })
}

async fn get_bundle_info(name: &str, directory: &Path) -> Result<(Option<String>, Option<usize>)> {
    let (dist_dir, extensions): (PathBuf, Vec<&str>) = match name {
        "Next.js" => (directory.join(".next/static/chunks"), vec!["js", "css"]),
        "rari" => (directory.join("dist/assets"), vec!["js", "css"]),
        _ => return Ok((None, None)),
    };

    if !dist_dir.exists() {
        return Ok((None, None));
    }

    let mut total_size = 0u64;
    let mut chunk_count = 0usize;

    scan_directory(&dist_dir, &extensions, &mut total_size, &mut chunk_count).await?;

    let bundle_size = if total_size > 0 {
        Some(format!("{:.2} kB", total_size as f64 / 1024.0))
    } else {
        None
    };

    Ok((bundle_size, Some(chunk_count)))
}

fn scan_directory<'a>(
    dir: &'a Path,
    extensions: &'a [&'a str],
    total_size: &'a mut u64,
    chunk_count: &'a mut usize,
) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
    Box::pin(async move {
        let mut entries = fs::read_dir(dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                scan_directory(&path, extensions, total_size, chunk_count).await?;
            } else if path.is_file()
                && let Some(ext) = path.extension()
                && extensions.contains(&ext.to_str().unwrap_or(""))
                && let Ok(metadata) = fs::metadata(&path).await
            {
                *total_size += metadata.len();
                *chunk_count += 1;
            }
        }
        Ok(())
    })
}

fn display_comparison(rari: &BuildResult, nextjs: &BuildResult) {
    println!("\n{}", "📊 Build Performance Comparison".bold());

    println!("\n⏱️  Build Times:");
    println!("  🦀 rari:     {:.2}s", rari.duration_ms / 1000.0);
    println!("  🟢 Next.js:  {:.2}s", nextjs.duration_ms / 1000.0);

    let time_diff = ((rari.duration_ms - nextjs.duration_ms) / nextjs.duration_ms) * 100.0;
    if time_diff < 0.0 {
        println!(
            "  {} rari builds {:.1}% faster",
            "📈".green(),
            time_diff.abs()
        );
    } else {
        println!("  {} rari builds {:.1}% slower", "📉".red(), time_diff);
    }

    println!("\n📦 Client Bundle Information:");
    println!("  🦀 rari:");
    println!(
        "     Size: {}",
        rari.bundle_size.as_deref().unwrap_or("Unknown")
    );
    println!(
        "     Files: {}",
        rari.chunk_count
            .map_or("Unknown".to_string(), |c| c.to_string())
    );
    println!("     Warnings: {}", rari.warnings);
    println!("     Errors: {}", rari.errors);

    println!("  🟢 Next.js:");
    println!(
        "     Size: {}",
        nextjs.bundle_size.as_deref().unwrap_or("Unknown")
    );
    println!(
        "     Files: {}",
        nextjs
            .chunk_count
            .map_or("Unknown".to_string(), |c| c.to_string())
    );
    println!("     Warnings: {}", nextjs.warnings);
    println!("     Errors: {}", nextjs.errors);
}

async fn save_results(results: &BenchmarkResults, base_dir: &Path) -> Result<()> {
    let results_dir = base_dir.join("results");
    fs::create_dir_all(&results_dir).await?;

    let date = time_utils::format_date(SystemTime::now());
    let filename = results_dir.join(format!("buildtimes-{}.json", date));

    let json = format!("{}\n", serde_json::to_string_pretty(results)?);
    fs::write(&filename, json).await?;

    println!(
        "\n{} Results saved to {}",
        "💾".dimmed(),
        filename.display()
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!(
        "{}",
        "🔨 rari vs Next.js Build Time Comparison".cyan().bold()
    );
    println!(
        "{}",
        "This benchmark compares build performance and bundle analysis\n".dimmed()
    );

    println!(
        "{}",
        "⚠️  This will run production builds which may take some time".yellow()
    );
    println!("{}", "Starting build comparison in 3 seconds...".dimmed());
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    let rari_dir = args.dir.join("rari-app");
    let nextjs_dir = args.dir.join("nextjs-app");

    let rari_result = run_build("rari", &rari_dir, "pnpm run build").await?;
    let nextjs_result = run_build("Next.js", &nextjs_dir, "pnpm run build").await?;

    display_comparison(&rari_result, &nextjs_result);

    let results = BenchmarkResults {
        timestamp: time_utils::format_timestamp(SystemTime::now()),
        rari: rari_result,
        nextjs: nextjs_result,
    };

    save_results(&results, &args.dir).await?;

    println!("\n{}", "🎉 Build comparison completed!".green().bold());

    Ok(())
}
