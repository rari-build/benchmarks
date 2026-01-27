mod time_utils;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tokio::fs;
use tokio::process::Command;

#[derive(Parser, Debug)]
#[command(name = "load-test")]
#[command(about = "Run load tests comparing rari and Next.js using oha")]
struct Args {
    #[arg(short, long, default_value = "30")]
    duration: u64,
    #[arg(short, long, default_value = "50")]
    connections: usize,
    #[arg(long, default_value = "3000")]
    rari_port: u16,
    #[arg(long, default_value = "3001")]
    nextjs_port: u16,
    #[arg(long, default_value = "results")]
    results_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RequestStats {
    #[serde(serialize_with = "time_utils::serialize_float_as_int_if_whole")]
    total: f64,
    average: f64,
    mean: f64,
    stddev: f64,
    #[serde(serialize_with = "time_utils::serialize_float_as_int_if_whole")]
    min: f64,
    #[serde(serialize_with = "time_utils::serialize_float_as_int_if_whole")]
    max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LatencyStats {
    average: f64,
    mean: f64,
    stddev: f64,
    #[serde(serialize_with = "time_utils::serialize_float_as_int_if_whole")]
    min: f64,
    #[serde(serialize_with = "time_utils::serialize_float_as_int_if_whole")]
    max: f64,
    #[serde(serialize_with = "time_utils::serialize_float_as_int_if_whole")]
    p50: f64,
    #[serde(serialize_with = "time_utils::serialize_float_as_int_if_whole")]
    p90: f64,
    #[serde(serialize_with = "time_utils::serialize_float_as_int_if_whole")]
    p95: f64,
    #[serde(serialize_with = "time_utils::serialize_float_as_int_if_whole")]
    p99: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThroughputStats {
    average: f64,
    mean: f64,
    stddev: f64,
    #[serde(serialize_with = "time_utils::serialize_float_as_int_if_whole")]
    min: f64,
    #[serde(serialize_with = "time_utils::serialize_float_as_int_if_whole")]
    max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoadTestResult {
    requests: RequestStats,
    latency: LatencyStats,
    throughput: ThroughputStats,
    errors: usize,
    timeouts: usize,
    duration: f64,
    start: String,
    finish: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkResults {
    timestamp: String,
    config: TestConfig,
    rari: LoadTestResult,
    nextjs: LoadTestResult,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestConfig {
    duration: u64,
    connections: usize,
}

async fn check_oha_installed() -> Result<()> {
    let output = Command::new("oha").arg("--version").output().await;

    match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("{} oha {}", "✅".green(), version.trim());
            Ok(())
        }
        _ => {
            anyhow::bail!(
                "oha is not installed. Install it with: cargo install oha\n\
                 Or visit: https://github.com/hatoo/oha"
            )
        }
    }
}

async fn check_server(name: &str, port: u16) -> Result<()> {
    let url = format!("http://localhost:{}", port);
    reqwest::get(&url)
        .await
        .context(format!("{} server is not responding at {}", name, url))?;
    println!("{} {} server is responding", "✅".green(), name);
    Ok(())
}

async fn run_load_test(
    name: &str,
    port: u16,
    duration: u64,
    connections: usize,
) -> Result<LoadTestResult> {
    println!("\n{} Load Testing {}", "🔥".bold(), name.bold());
    let url = format!("http://localhost:{}", port);
    println!("  {} {}", "URL:".dimmed(), url);
    println!(
        "  {} {}s, Connections: {}",
        "Duration:".dimmed(),
        duration,
        connections
    );

    let start_time = SystemTime::now();
    let start_str = time_utils::format_timestamp(start_time);

    let output = Command::new("oha")
        .arg(&url)
        .arg("-z")
        .arg(format!("{}s", duration))
        .arg("-c")
        .arg(connections.to_string())
        .arg("--no-tui")
        .arg("--output-format")
        .arg("json")
        .output()
        .await
        .context("Failed to execute oha")?;

    let finish_time = SystemTime::now();
    let finish_str = time_utils::format_timestamp(finish_time);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("oha failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout).context("Failed to parse oha JSON output")?;

    let summary = &json["summary"];
    let percentiles = &json["latencyPercentiles"];

    let total_requests = (summary["successRate"].as_f64().unwrap_or(1.0)
        * summary["requestsPerSec"].as_f64().unwrap_or(0.0)
        * summary["total"].as_f64().unwrap_or(0.0)) as f64;

    let requests = RequestStats {
        total: total_requests,
        average: summary["requestsPerSec"].as_f64().unwrap_or(0.0),
        mean: summary["requestsPerSec"].as_f64().unwrap_or(0.0),
        stddev: 0.0,
        min: 0.0,
        max: 0.0,
    };

    let latency = LatencyStats {
        average: summary["average"].as_f64().unwrap_or(0.0) * 1000.0,
        mean: summary["average"].as_f64().unwrap_or(0.0) * 1000.0,
        stddev: 0.0,
        min: summary["fastest"].as_f64().unwrap_or(0.0) * 1000.0,
        max: summary["slowest"].as_f64().unwrap_or(0.0) * 1000.0,
        p50: percentiles["p50"].as_f64().unwrap_or(0.0) * 1000.0,
        p90: percentiles["p90"].as_f64().unwrap_or(0.0) * 1000.0,
        p95: percentiles["p95"].as_f64().unwrap_or(0.0) * 1000.0,
        p99: percentiles["p99"].as_f64().unwrap_or(0.0) * 1000.0,
    };

    let duration_secs = summary["total"].as_f64().unwrap_or(duration as f64);

    let throughput = ThroughputStats {
        average: summary["sizePerSec"].as_f64().unwrap_or(0.0),
        mean: summary["sizePerSec"].as_f64().unwrap_or(0.0),
        stddev: 0.0,
        min: 0.0,
        max: 0.0,
    };

    let success_rate = summary["successRate"].as_f64().unwrap_or(1.0);
    let total = total_requests as usize;
    let errors = ((1.0 - success_rate) * total as f64) as usize;

    println!(
        "  {} Completed: {} requests ({} successful, {} failed)",
        "✅".green(),
        total,
        total - errors,
        errors
    );

    Ok(LoadTestResult {
        requests,
        latency,
        throughput,
        errors,
        timeouts: 0,
        duration: duration_secs,
        start: start_str,
        finish: finish_str,
    })
}

fn display_comparison(rari: &LoadTestResult, nextjs: &LoadTestResult) {
    println!("\n{}", "📊 Load Test Comparison".bold());

    println!("\n📈 Throughput (req/sec):");
    println!("  🦀 rari:     {:.2}", rari.requests.average);
    println!("  🟢 Next.js:  {:.2}", nextjs.requests.average);

    let throughput_diff =
        ((rari.requests.average - nextjs.requests.average) / nextjs.requests.average) * 100.0;
    if throughput_diff > 0.0 {
        println!(
            "  {} rari handles {:.1}% more requests/sec",
            "📈".green(),
            throughput_diff
        );
    } else {
        println!(
            "  {} rari handles {:.1}% fewer requests/sec",
            "📉".red(),
            throughput_diff.abs()
        );
    }

    println!("\n⏱️  Latency (ms):");
    println!(
        "  🦀 rari:     {:.2}ms (P95: {:.2}ms)",
        rari.latency.mean, rari.latency.p95
    );
    println!(
        "  🟢 Next.js:  {:.2}ms (P95: {:.2}ms)",
        nextjs.latency.mean, nextjs.latency.p95
    );

    let latency_diff = ((rari.latency.mean - nextjs.latency.mean) / nextjs.latency.mean) * 100.0;
    if latency_diff < 0.0 {
        println!(
            "  {} rari is {:.1}% faster response time",
            "📈".green(),
            latency_diff.abs()
        );
    } else {
        println!(
            "  {} rari is {:.1}% slower response time",
            "📉".red(),
            latency_diff
        );
    }

    println!("\n🚨 Errors:");
    println!(
        "  🦀 rari:     {} errors, {} timeouts",
        rari.errors, rari.timeouts
    );
    println!(
        "  🟢 Next.js:  {} errors, {} timeouts",
        nextjs.errors, nextjs.timeouts
    );
}

async fn save_results(results: &BenchmarkResults, results_dir: &PathBuf) -> Result<()> {
    fs::create_dir_all(results_dir).await?;

    let now = SystemTime::now();
    let date = time_utils::format_date(now);
    let filename = results_dir.join(format!("loadtest-{}.json", date));

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

    println!("{}", "🔥 rari vs Next.js Load Test".cyan().bold());
    println!(
        "{}",
        "This test measures concurrent request handling performance\n".dimmed()
    );

    if let Err(e) = check_oha_installed().await {
        eprintln!("{} {}", "❌".red(), e);
        std::process::exit(1);
    }

    if let Err(e) = check_server("rari", args.rari_port).await {
        eprintln!("{} {}", "❌".red(), e);
        eprintln!("Please start the rari server with: cd rari-app && pnpm dev");
        std::process::exit(1);
    }

    if let Err(e) = check_server("Next.js", args.nextjs_port).await {
        eprintln!("{} {}", "❌".red(), e);
        eprintln!("Please start the Next.js server with: cd nextjs-app && pnpm dev");
        std::process::exit(1);
    }

    println!(
        "\n{}",
        "⚠️  This test will generate significant load on both servers".yellow()
    );
    println!("{}", "Starting load test in 3 seconds...".dimmed());
    tokio::time::sleep(Duration::from_secs(3)).await;

    let rari_result =
        run_load_test("rari", args.rari_port, args.duration, args.connections).await?;

    println!("\n{}", "Pausing between tests...".dimmed());
    tokio::time::sleep(Duration::from_secs(2)).await;

    let nextjs_result =
        run_load_test("Next.js", args.nextjs_port, args.duration, args.connections).await?;

    display_comparison(&rari_result, &nextjs_result);

    let results = BenchmarkResults {
        timestamp: time_utils::format_timestamp(SystemTime::now()),
        config: TestConfig {
            duration: args.duration,
            connections: args.connections,
        },
        rari: rari_result,
        nextjs: nextjs_result,
    };

    save_results(&results, &args.results_dir).await?;

    println!("\n{}", "🎉 Load test completed!".green().bold());

    Ok(())
}
