mod time_utils;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};
use tabled::{Table, Tabled};
use tokio::fs;
use tokio::process::Command;

const CONTENT_MARKER: &[u8] = b"data-bench-stream=\"resolved\"";

#[derive(Parser, Debug)]
#[command(name = "streaming")]
#[command(about = "Benchmark Suspense streaming (/stream) for rari vs Next.js")]
struct Args {
    /// Number of per-chunk profile runs per framework
    #[arg(short, long, default_value = "5")]
    runs: usize,

    /// Request timeout for a single streaming profile (seconds)
    #[arg(long, default_value = "15")]
    timeout: u64,

    /// oha duration for streaming throughput (seconds)
    #[arg(short, long, default_value = "15")]
    duration: u64,

    /// Concurrent connections for streaming throughput (lighter than flat load)
    #[arg(short, long, default_value = "25")]
    connections: usize,

    /// Skip oha throughput measurement
    #[arg(long, default_value_t = false)]
    profile_only: bool,

    #[arg(long, default_value = "3000")]
    rari_port: u16,

    #[arg(long, default_value = "3001")]
    nextjs_port: u16,

    #[arg(long, default_value = "results")]
    results_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InterChunkGaps {
    mean: f64,
    p50: f64,
    p95: f64,
    max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProgressiveBytes {
    #[serde(rename = "500ms")]
    at_500ms: u64,
    #[serde(rename = "1000ms")]
    at_1000ms: u64,
    #[serde(rename = "2000ms")]
    at_2000ms: u64,
    #[serde(rename = "5000ms")]
    at_5000ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProgressiveResolved {
    #[serde(rename = "500ms")]
    at_500ms: usize,
    #[serde(rename = "1000ms")]
    at_1000ms: usize,
    #[serde(rename = "2000ms")]
    at_2000ms: usize,
    #[serde(rename = "5000ms")]
    at_5000ms: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StreamProfile {
    ttfb_ms: f64,
    #[serde(rename = "firstContentChunk_ms")]
    first_content_chunk_ms: Option<f64>,
    #[serde(rename = "lastByte_ms")]
    last_byte_ms: f64,
    chunks: usize,
    #[serde(rename = "resolvedCards")]
    resolved_cards: usize,
    #[serde(rename = "interChunkGap_ms")]
    inter_chunk_gap_ms: InterChunkGaps,
    #[serde(rename = "skeletonDuration_ms")]
    skeleton_duration_ms: f64,
    #[serde(rename = "progressiveBytes")]
    progressive_bytes: ProgressiveBytes,
    #[serde(rename = "progressiveResolved")]
    progressive_resolved: ProgressiveResolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThroughputStats {
    #[serde(rename = "req_s")]
    req_s: f64,
    #[serde(rename = "latency_avg_ms")]
    latency_avg_ms: f64,
    #[serde(rename = "latency_p95_ms")]
    latency_p95_ms: f64,
    errors: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FrameworkStreamingResult {
    target: String,
    path: String,
    runs: usize,
    successful: usize,
    profile: Option<StreamProfile>,
    throughput: Option<ThroughputStats>,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StreamingBenchmarkResults {
    timestamp: String,
    config: StreamingConfig,
    rari: FrameworkStreamingResult,
    nextjs: FrameworkStreamingResult,
    comparison: Option<StreamingComparison>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StreamingConfig {
    runs: usize,
    timeout_secs: u64,
    duration_secs: u64,
    connections: usize,
    profile_only: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct StreamingComparison {
    #[serde(rename = "ttfb_ratio")]
    ttfb_ratio: f64,
    #[serde(rename = "lastByte_ratio")]
    last_byte_ratio: f64,
    #[serde(rename = "throughput_ratio")]
    throughput_ratio: Option<f64>,
}

#[derive(Tabled)]
struct ProfileRow {
    #[tabled(rename = "Metric")]
    metric: String,
    #[tabled(rename = "rari")]
    rari: String,
    #[tabled(rename = "Next.js")]
    nextjs: String,
    #[tabled(rename = "Ratio")]
    ratio: String,
}

struct ChunkSample {
    ms: f64,
    bytes: usize,
    resolved_so_far: usize,
}

fn count_markers(haystack: &[u8]) -> usize {
    if haystack.len() < CONTENT_MARKER.len() {
        return 0;
    }
    haystack
        .windows(CONTENT_MARKER.len())
        .filter(|window| *window == CONTENT_MARKER)
        .count()
}

fn build_http_client(timeout: Duration) -> Result<reqwest::Client> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::ACCEPT_ENCODING,
        reqwest::header::HeaderValue::from_static("identity"),
    );

    reqwest::Client::builder()
        .timeout(timeout)
        .default_headers(headers)
        .build()
        .context("Failed to build HTTP client")
}

async fn check_server(name: &str, port: u16) -> Result<()> {
    let url = format!("http://localhost:{}/stream", port);
    let client = build_http_client(Duration::from_secs(15))?;
    client.get(&url).send().await.context(format!(
        "{} streaming route is not responding at {}",
        name, url
    ))?;
    println!("{} {} /stream is responding", "✅".green(), name);
    Ok(())
}

async fn check_oha_installed() -> Result<()> {
    let output = Command::new("oha").arg("--version").output().await;
    match output {
        Ok(output) if output.status.success() => Ok(()),
        _ => anyhow::bail!(
            "oha is not installed. Install it with: cargo install oha\n\
             Or visit: https://github.com/hatoo/oha"
        ),
    }
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let index = ((p * (sorted.len() - 1) as f64).round()) as usize;
    sorted[index.min(sorted.len() - 1)]
}

fn median_f64(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    sorted[sorted.len() / 2]
}

fn median_usize(values: &[usize]) -> usize {
    if values.is_empty() {
        return 0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    sorted[sorted.len() / 2]
}

async fn profile_once(client: &reqwest::Client, url: &str) -> Result<StreamProfile> {
    let start = Instant::now();
    let response = client.get(url).send().await.context("request failed")?;

    if !response.status().is_success() {
        anyhow::bail!("HTTP {}", response.status());
    }

    let mut chunks = Vec::new();
    let mut body = Vec::new();
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let bytes = item.context("failed reading stream chunk")?;
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        body.extend_from_slice(&bytes);
        chunks.push(ChunkSample {
            ms: elapsed_ms,
            bytes: bytes.len(),
            resolved_so_far: count_markers(&body),
        });
    }

    if chunks.is_empty() {
        anyhow::bail!("No chunks received");
    }

    let ttfb = chunks[0].ms;
    let last_byte = chunks[chunks.len() - 1].ms;
    let resolved_cards = chunks[chunks.len() - 1].resolved_so_far;

    let mut gaps = Vec::new();
    for i in 1..chunks.len() {
        gaps.push(chunks[i].ms - chunks[i - 1].ms);
    }
    gaps.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let gap_mean = if gaps.is_empty() {
        0.0
    } else {
        gaps.iter().sum::<f64>() / gaps.len() as f64
    };

    let first_content = chunks.iter().find(|c| c.resolved_so_far > 0).map(|c| c.ms);

    let progressive_bytes = |limit_ms: f64| -> u64 {
        chunks
            .iter()
            .filter(|c| c.ms <= limit_ms)
            .map(|c| c.bytes as u64)
            .sum()
    };

    let progressive_resolved = |limit_ms: f64| -> usize {
        chunks
            .iter()
            .rev()
            .find(|c| c.ms <= limit_ms)
            .map(|c| c.resolved_so_far)
            .unwrap_or(0)
    };

    Ok(StreamProfile {
        ttfb_ms: ttfb,
        first_content_chunk_ms: first_content,
        last_byte_ms: last_byte,
        chunks: chunks.len(),
        resolved_cards,
        inter_chunk_gap_ms: InterChunkGaps {
            mean: (gap_mean * 100.0).round() / 100.0,
            p50: percentile(&gaps, 0.50),
            p95: percentile(&gaps, 0.95),
            max: gaps.last().copied().unwrap_or(0.0),
        },
        skeleton_duration_ms: last_byte - ttfb,
        progressive_bytes: ProgressiveBytes {
            at_500ms: progressive_bytes(500.0),
            at_1000ms: progressive_bytes(1000.0),
            at_2000ms: progressive_bytes(2000.0),
            at_5000ms: progressive_bytes(5000.0),
        },
        progressive_resolved: ProgressiveResolved {
            at_500ms: progressive_resolved(500.0),
            at_1000ms: progressive_resolved(1000.0),
            at_2000ms: progressive_resolved(2000.0),
            at_5000ms: progressive_resolved(5000.0),
        },
    })
}

fn aggregate_profiles(profiles: &[StreamProfile]) -> StreamProfile {
    StreamProfile {
        ttfb_ms: median_f64(&profiles.iter().map(|p| p.ttfb_ms).collect::<Vec<_>>()),
        first_content_chunk_ms: {
            let values: Vec<f64> = profiles
                .iter()
                .filter_map(|p| p.first_content_chunk_ms)
                .collect();
            if values.is_empty() {
                None
            } else {
                Some(median_f64(&values))
            }
        },
        last_byte_ms: median_f64(&profiles.iter().map(|p| p.last_byte_ms).collect::<Vec<_>>()),
        chunks: median_usize(&profiles.iter().map(|p| p.chunks).collect::<Vec<_>>()),
        resolved_cards: median_usize(
            &profiles
                .iter()
                .map(|p| p.resolved_cards)
                .collect::<Vec<_>>(),
        ),
        inter_chunk_gap_ms: InterChunkGaps {
            mean: (median_f64(
                &profiles
                    .iter()
                    .map(|p| p.inter_chunk_gap_ms.mean)
                    .collect::<Vec<_>>(),
            ) * 100.0)
                .round()
                / 100.0,
            p50: median_f64(
                &profiles
                    .iter()
                    .map(|p| p.inter_chunk_gap_ms.p50)
                    .collect::<Vec<_>>(),
            ),
            p95: median_f64(
                &profiles
                    .iter()
                    .map(|p| p.inter_chunk_gap_ms.p95)
                    .collect::<Vec<_>>(),
            ),
            max: median_f64(
                &profiles
                    .iter()
                    .map(|p| p.inter_chunk_gap_ms.max)
                    .collect::<Vec<_>>(),
            ),
        },
        skeleton_duration_ms: median_f64(
            &profiles
                .iter()
                .map(|p| p.skeleton_duration_ms)
                .collect::<Vec<_>>(),
        ),
        progressive_bytes: ProgressiveBytes {
            at_500ms: median_f64(
                &profiles
                    .iter()
                    .map(|p| p.progressive_bytes.at_500ms as f64)
                    .collect::<Vec<_>>(),
            )
            .round() as u64,
            at_1000ms: median_f64(
                &profiles
                    .iter()
                    .map(|p| p.progressive_bytes.at_1000ms as f64)
                    .collect::<Vec<_>>(),
            )
            .round() as u64,
            at_2000ms: median_f64(
                &profiles
                    .iter()
                    .map(|p| p.progressive_bytes.at_2000ms as f64)
                    .collect::<Vec<_>>(),
            )
            .round() as u64,
            at_5000ms: median_f64(
                &profiles
                    .iter()
                    .map(|p| p.progressive_bytes.at_5000ms as f64)
                    .collect::<Vec<_>>(),
            )
            .round() as u64,
        },
        progressive_resolved: ProgressiveResolved {
            at_500ms: median_usize(
                &profiles
                    .iter()
                    .map(|p| p.progressive_resolved.at_500ms)
                    .collect::<Vec<_>>(),
            ),
            at_1000ms: median_usize(
                &profiles
                    .iter()
                    .map(|p| p.progressive_resolved.at_1000ms)
                    .collect::<Vec<_>>(),
            ),
            at_2000ms: median_usize(
                &profiles
                    .iter()
                    .map(|p| p.progressive_resolved.at_2000ms)
                    .collect::<Vec<_>>(),
            ),
            at_5000ms: median_usize(
                &profiles
                    .iter()
                    .map(|p| p.progressive_resolved.at_5000ms)
                    .collect::<Vec<_>>(),
            ),
        },
    }
}

async fn profile_target(
    name: &str,
    port: u16,
    runs: usize,
    timeout: Duration,
) -> FrameworkStreamingResult {
    let url = format!("http://localhost:{}/stream", port);
    println!("\n{} Profiling {} ({})", "📡".bold(), name.bold(), url);

    let client = match build_http_client(timeout) {
        Ok(c) => c,
        Err(e) => {
            return FrameworkStreamingResult {
                target: name.to_string(),
                path: "/stream".to_string(),
                runs,
                successful: 0,
                profile: None,
                throughput: None,
                error: Some(e.to_string()),
            };
        }
    };

    let mut profiles = Vec::new();

    for run in 1..=runs {
        match profile_once(&client, &url).await {
            Ok(profile) => {
                println!(
                    "  Run {}/{}: ttfb={:.0}ms firstContent={} lastByte={:.0}ms resolved={}/10 frames={}",
                    run,
                    runs,
                    profile.ttfb_ms,
                    profile
                        .first_content_chunk_ms
                        .map(|v| format!("{:.0}ms", v))
                        .unwrap_or_else(|| "—".to_string()),
                    profile.last_byte_ms,
                    profile.resolved_cards,
                    profile.chunks
                );
                profiles.push(profile);
            }
            Err(e) => {
                println!("  Run {}/{}: ERROR {}", run, runs, e);
            }
        }
    }

    if profiles.is_empty() {
        return FrameworkStreamingResult {
            target: name.to_string(),
            path: "/stream".to_string(),
            runs,
            successful: 0,
            profile: None,
            throughput: None,
            error: Some("All profile runs failed".to_string()),
        };
    }

    FrameworkStreamingResult {
        target: name.to_string(),
        path: "/stream".to_string(),
        runs,
        successful: profiles.len(),
        profile: Some(aggregate_profiles(&profiles)),
        throughput: None,
        error: None,
    }
}

async fn measure_throughput(
    name: &str,
    port: u16,
    duration: u64,
    connections: usize,
) -> Result<ThroughputStats> {
    println!(
        "\n{} Throughput {} ({}s, {} connections)",
        "🔥".bold(),
        name.bold(),
        duration,
        connections
    );

    let url = format!("http://localhost:{}/stream", port);
    let output = Command::new("oha")
        .arg(&url)
        .arg("-z")
        .arg(format!("{}s", duration))
        .arg("-c")
        .arg(connections.to_string())
        .arg("-H")
        .arg("Accept-Encoding: identity")
        .arg("--no-tui")
        .arg("--output-format")
        .arg("json")
        .output()
        .await
        .context("Failed to execute oha")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("oha failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout).context("Failed to parse oha JSON output")?;
    let summary = &json["summary"];
    let percentiles = &json["latencyPercentiles"];

    let req_s = summary["requestsPerSec"].as_f64().unwrap_or(0.0);
    let latency_avg_ms = summary["average"].as_f64().unwrap_or(0.0) * 1000.0;
    let latency_p95_ms = percentiles["p95"].as_f64().unwrap_or(0.0) * 1000.0;
    let success_rate = summary["successRate"].as_f64().unwrap_or(1.0);
    let total = (req_s * summary["total"].as_f64().unwrap_or(duration as f64)) as usize;
    let errors = ((1.0 - success_rate) * total as f64) as usize;

    println!(
        "  {} {:.2} req/s, avg {:.0}ms, p95 {:.0}ms",
        "✅".green(),
        req_s,
        latency_avg_ms,
        latency_p95_ms
    );

    Ok(ThroughputStats {
        req_s,
        latency_avg_ms,
        latency_p95_ms,
        errors,
    })
}

fn display_results(rari: &FrameworkStreamingResult, nextjs: &FrameworkStreamingResult) {
    println!("\n{}", "📈 Streaming Comparison".bold());
    println!(
        "{}",
        "Primary: TTFB, first content, throughput. Last-byte is delay-dominated (~1000ms cards)."
            .dimmed()
    );

    let (Some(rp), Some(np)) = (&rari.profile, &nextjs.profile) else {
        println!("{}", "❌ Missing profile data for comparison".red());
        return;
    };

    let ratio = |a: f64, b: f64| if b == 0.0 { 0.0 } else { a / b };

    let rows = vec![
        ProfileRow {
            metric: "TTFB (ms)".to_string(),
            rari: format!("{:.0}", rp.ttfb_ms),
            nextjs: format!("{:.0}", np.ttfb_ms),
            ratio: format!("{:.2}x", ratio(np.ttfb_ms, rp.ttfb_ms)),
        },
        ProfileRow {
            metric: "First content (ms)".to_string(),
            rari: rp
                .first_content_chunk_ms
                .map(|v| format!("{:.0}", v))
                .unwrap_or_else(|| "—".to_string()),
            nextjs: np
                .first_content_chunk_ms
                .map(|v| format!("{:.0}", v))
                .unwrap_or_else(|| "—".to_string()),
            ratio: match (rp.first_content_chunk_ms, np.first_content_chunk_ms) {
                (Some(a), Some(b)) => format!("{:.2}x", ratio(b, a)),
                _ => "—".to_string(),
            },
        },
        ProfileRow {
            metric: "Last byte (ms)".to_string(),
            rari: format!("{:.0}", rp.last_byte_ms),
            nextjs: format!("{:.0}", np.last_byte_ms),
            ratio: format!("{:.2}x", ratio(np.last_byte_ms, rp.last_byte_ms)),
        },
        ProfileRow {
            metric: "Resolved cards".to_string(),
            rari: rp.resolved_cards.to_string(),
            nextjs: np.resolved_cards.to_string(),
            ratio: "—".to_string(),
        },
        ProfileRow {
            metric: "HTTP frames".to_string(),
            rari: rp.chunks.to_string(),
            nextjs: np.chunks.to_string(),
            ratio: "—".to_string(),
        },
        ProfileRow {
            metric: "Gap p95 (ms)".to_string(),
            rari: format!("{:.0}", rp.inter_chunk_gap_ms.p95),
            nextjs: format!("{:.0}", np.inter_chunk_gap_ms.p95),
            ratio: format!(
                "{:.2}x",
                ratio(np.inter_chunk_gap_ms.p95, rp.inter_chunk_gap_ms.p95)
            ),
        },
        ProfileRow {
            metric: "Resolved @500ms".to_string(),
            rari: rp.progressive_resolved.at_500ms.to_string(),
            nextjs: np.progressive_resolved.at_500ms.to_string(),
            ratio: "—".to_string(),
        },
        ProfileRow {
            metric: "Resolved @1000ms".to_string(),
            rari: rp.progressive_resolved.at_1000ms.to_string(),
            nextjs: np.progressive_resolved.at_1000ms.to_string(),
            ratio: "—".to_string(),
        },
    ];

    println!("\n{}", Table::new(rows));

    if let (Some(rt), Some(nt)) = (&rari.throughput, &nextjs.throughput) {
        println!("\n{}", "🔥 Streaming Throughput".bold());
        println!("  🦀 rari:     {:.2} req/s", rt.req_s);
        println!("  🟢 Next.js:  {:.2} req/s", nt.req_s);
        if nt.req_s > 0.0 {
            println!("  Ratio: {:.2}x", rt.req_s / nt.req_s);
        }
    }
}

fn build_comparison(
    rari: &FrameworkStreamingResult,
    nextjs: &FrameworkStreamingResult,
) -> Option<StreamingComparison> {
    let rp = rari.profile.as_ref()?;
    let np = nextjs.profile.as_ref()?;

    let throughput_ratio = match (&rari.throughput, &nextjs.throughput) {
        (Some(rt), Some(nt)) if nt.req_s > 0.0 => Some(rt.req_s / nt.req_s),
        _ => None,
    };

    Some(StreamingComparison {
        ttfb_ratio: if rp.ttfb_ms > 0.0 {
            np.ttfb_ms / rp.ttfb_ms
        } else {
            0.0
        },
        last_byte_ratio: if rp.last_byte_ms > 0.0 {
            np.last_byte_ms / rp.last_byte_ms
        } else {
            0.0
        },
        throughput_ratio,
    })
}

async fn save_results(results: &StreamingBenchmarkResults, results_dir: &PathBuf) -> Result<()> {
    fs::create_dir_all(results_dir).await?;

    let date = time_utils::format_date(SystemTime::now());
    let filename = results_dir.join(format!("streaming-{}.json", date));
    let json = format!("{}\n", serde_json::to_string_pretty(results)?);

    fs::write(&filename, &json).await?;
    fs::write(results_dir.join("streaming-latest.json"), &json).await?;

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

    println!("{}", "🌊 rari vs Next.js Streaming Benchmark".cyan().bold());
    println!(
        "{}",
        "Measures Suspense streaming on /stream: TTFB, resolved cards, inter-chunk gaps, throughput\n"
            .dimmed()
    );

    if let Err(e) = check_server("rari", args.rari_port).await {
        eprintln!("{} {}", "❌".red(), e);
        eprintln!("Please start the rari server with: just start-rari");
        std::process::exit(1);
    }

    if let Err(e) = check_server("Next.js", args.nextjs_port).await {
        eprintln!("{} {}", "❌".red(), e);
        eprintln!("Please start the Next.js server with: just start-nextjs");
        std::process::exit(1);
    }

    if !args.profile_only {
        if let Err(e) = check_oha_installed().await {
            eprintln!("{} {}", "❌".red(), e);
            std::process::exit(1);
        }
    }

    let timeout = Duration::from_secs(args.timeout);

    let mut rari = profile_target("rari", args.rari_port, args.runs, timeout).await;
    let mut nextjs = profile_target("Next.js", args.nextjs_port, args.runs, timeout).await;

    if !args.profile_only {
        match measure_throughput("rari", args.rari_port, args.duration, args.connections).await {
            Ok(stats) => rari.throughput = Some(stats),
            Err(e) => println!("  {} rari throughput failed: {}", "❌".red(), e),
        }

        println!("\n{}", "Pausing between throughput tests...".dimmed());
        tokio::time::sleep(Duration::from_secs(2)).await;

        match measure_throughput("Next.js", args.nextjs_port, args.duration, args.connections).await
        {
            Ok(stats) => nextjs.throughput = Some(stats),
            Err(e) => println!("  {} Next.js throughput failed: {}", "❌".red(), e),
        }
    }

    display_results(&rari, &nextjs);
    let comparison = build_comparison(&rari, &nextjs);

    let results = StreamingBenchmarkResults {
        timestamp: time_utils::format_timestamp(SystemTime::now()),
        config: StreamingConfig {
            runs: args.runs,
            timeout_secs: args.timeout,
            duration_secs: args.duration,
            connections: args.connections,
            profile_only: args.profile_only,
        },
        rari,
        nextjs,
        comparison,
    };

    save_results(&results, &args.results_dir).await?;

    println!("\n{}", "🎉 Streaming benchmark completed!".green().bold());

    Ok(())
}
