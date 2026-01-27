mod time_utils;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};
use tabled::{Table, Tabled};
use tokio::fs;

#[derive(Parser, Debug)]
#[command(name = "performance")]
#[command(about = "Run performance benchmarks comparing rari and Next.js")]
struct Args {
    #[arg(short, long, default_value = "50")]
    warmup: usize,
    #[arg(short, long, default_value = "20")]
    requests: usize,
    #[arg(long, default_value = "3000")]
    rari_port: u16,
    #[arg(long, default_value = "3001")]
    nextjs_port: u16,
    #[arg(long, default_value = "results")]
    results_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerformanceMetrics {
    min: f64,
    max: f64,
    avg: f64,
    p50: f64,
    p95: f64,
    p99: f64,
    #[serde(rename = "avgSize")]
    avg_size: usize,
    errors: usize,
    #[serde(
        rename = "successRate",
        serialize_with = "time_utils::serialize_float_as_int_if_whole"
    )]
    success_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkResults {
    timestamp: String,
    rari: HashMap<String, PerformanceMetrics>,
    nextjs: HashMap<String, PerformanceMetrics>,
    summary: TestSummary,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestSummary {
    #[serde(rename = "testRequests")]
    test_requests: usize,
    #[serde(rename = "warmupRequests")]
    warmup_requests: usize,
    scenarios: usize,
}

#[derive(Debug, Clone)]
struct Scenario {
    path: String,
    name: String,
}

#[derive(Tabled)]
struct ComparisonRow {
    #[tabled(rename = "Scenario")]
    scenario: String,
    #[tabled(rename = "rari (ms)")]
    rari_ms: String,
    #[tabled(rename = "Next.js (ms)")]
    nextjs_ms: String,
    #[tabled(rename = "Difference")]
    difference: String,
    #[tabled(rename = "Winner")]
    winner: String,
}

async fn check_server(name: &str, port: u16) -> Result<()> {
    let url = format!("http://localhost:{}", port);
    reqwest::get(&url)
        .await
        .context(format!("{} server is not responding at {}", name, url))?;
    println!("{} {} server is responding", "✅".green(), name);
    Ok(())
}

async fn measure_request(url: &str, warmup: usize, requests: usize) -> Result<PerformanceMetrics> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    println!("  Testing {}...", url);

    for _ in 0..warmup {
        let _ = client.get(url).send().await;
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    let mut times = Vec::new();
    let mut sizes = Vec::new();
    let mut errors = 0;

    for _ in 0..requests {
        let start = Instant::now();

        match client.get(url).send().await {
            Ok(response) if response.status().is_success() => {
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;

                match response.text().await {
                    Ok(text) => {
                        times.push(elapsed);
                        sizes.push(text.len());
                    }
                    Err(_) => errors += 1,
                }
            }
            _ => errors += 1,
        }

        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    if times.is_empty() {
        anyhow::bail!("No successful requests");
    }

    let mut sorted_times = times.clone();
    sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let sum: f64 = times.iter().sum();
    let avg = sum / times.len() as f64;
    let avg_size = sizes.iter().sum::<usize>() / sizes.len();
    let success_rate = ((requests - errors) as f64 / requests as f64) * 100.0;

    Ok(PerformanceMetrics {
        min: sorted_times[0],
        max: sorted_times[sorted_times.len() - 1],
        avg,
        p50: percentile(&sorted_times, 0.50),
        p95: percentile(&sorted_times, 0.95),
        p99: percentile(&sorted_times, 0.99),
        avg_size,
        errors,
        success_rate,
    })
}

fn percentile(sorted_data: &[f64], p: f64) -> f64 {
    let index = (p * (sorted_data.len() - 1) as f64) as usize;
    sorted_data[index]
}

async fn benchmark_framework(
    name: &str,
    port: u16,
    scenarios: &[Scenario],
    warmup: usize,
    requests: usize,
) -> Result<HashMap<String, PerformanceMetrics>> {
    println!(
        "\n{} Benchmarking {} (port {})",
        "🚀".bold(),
        name.bold(),
        port
    );

    let mut results = HashMap::new();

    for scenario in scenarios {
        let url = format!("http://localhost:{}{}", port, scenario.path);
        println!("\n📊 {}", scenario.name);

        match measure_request(&url, warmup, requests).await {
            Ok(metrics) => {
                println!(
                    "  {} Avg: {:.2}ms, P95: {:.2}ms, Size: {}b",
                    "✅".green(),
                    metrics.avg,
                    metrics.p95,
                    metrics.avg_size
                );
                results.insert(scenario.name.clone(), metrics);
            }
            Err(e) => {
                println!("  {} Failed: {}", "❌".red(), e);
            }
        }
    }

    Ok(results)
}

fn display_comparison(
    scenarios: &[Scenario],
    rari_results: &HashMap<String, PerformanceMetrics>,
    nextjs_results: &HashMap<String, PerformanceMetrics>,
) {
    println!("\n{}", "📈 Performance Comparison".bold());

    let mut rows = Vec::new();

    for scenario in scenarios {
        if let (Some(rari), Some(nextjs)) = (
            rari_results.get(&scenario.name),
            nextjs_results.get(&scenario.name),
        ) {
            let diff = ((rari.avg - nextjs.avg) / nextjs.avg) * 100.0;
            let winner = if rari.avg < nextjs.avg {
                "🦀 rari"
            } else {
                "🟢 Next.js"
            };
            let diff_str = if diff > 0.0 {
                format!("+{:.1}%", diff)
            } else {
                format!("{:.1}%", diff)
            };

            rows.push(ComparisonRow {
                scenario: scenario.name.clone(),
                rari_ms: format!("{:.2}", rari.avg),
                nextjs_ms: format!("{:.2}", nextjs.avg),
                difference: diff_str,
                winner: winner.to_string(),
            });
        }
    }

    let table = Table::new(rows).to_string();
    println!("\n{}", table);
}

fn calculate_summary(
    scenarios: &[Scenario],
    rari_results: &HashMap<String, PerformanceMetrics>,
    nextjs_results: &HashMap<String, PerformanceMetrics>,
) {
    let valid_scenarios: Vec<_> = scenarios
        .iter()
        .filter(|s| rari_results.contains_key(&s.name) && nextjs_results.contains_key(&s.name))
        .collect();

    if valid_scenarios.is_empty() {
        println!("\n{}", "❌ No valid results to compare".red());
        return;
    }

    let rari_avg: f64 = valid_scenarios
        .iter()
        .map(|s| rari_results[&s.name].avg)
        .sum::<f64>()
        / valid_scenarios.len() as f64;

    let nextjs_avg: f64 = valid_scenarios
        .iter()
        .map(|s| nextjs_results[&s.name].avg)
        .sum::<f64>()
        / valid_scenarios.len() as f64;

    let improvement = ((nextjs_avg - rari_avg) / nextjs_avg) * 100.0;

    println!("\n{}", "📊 Summary".bold());
    println!("Average Response Time:");
    println!("  🦀 rari:     {:.2}ms", rari_avg);
    println!("  🟢 Next.js:  {:.2}ms", nextjs_avg);

    if improvement > 0.0 {
        println!("  {} rari is {:.1}% faster", "📈".green(), improvement);
    } else {
        println!("  {} rari is {:.1}% slower", "📉".red(), improvement.abs());
    }
}

async fn save_results(results: &BenchmarkResults, results_dir: &PathBuf) -> Result<()> {
    fs::create_dir_all(results_dir).await?;

    let now = SystemTime::now();
    let date = time_utils::format_date(now);
    let filename = results_dir.join(format!("performance-{}.json", date));

    let json = format!("{}\n", serde_json::to_string_pretty(results)?);
    fs::write(&filename, &json).await?;

    let latest = results_dir.join("latest.json");
    fs::write(&latest, &json).await?;

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
        "🏁 rari vs Next.js Performance Benchmark".cyan().bold()
    );
    println!(
        "{}",
        "This benchmark compares server-side rendering performance\n".dimmed()
    );

    let scenarios = vec![Scenario {
        path: "/".to_string(),
        name: "Homepage (All Components)".to_string(),
    }];

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

    println!("\n{}", "Starting benchmark in 3 seconds...".dimmed());
    tokio::time::sleep(Duration::from_secs(3)).await;

    let rari_results = benchmark_framework(
        "rari",
        args.rari_port,
        &scenarios,
        args.warmup,
        args.requests,
    )
    .await?;
    let nextjs_results = benchmark_framework(
        "Next.js",
        args.nextjs_port,
        &scenarios,
        args.warmup,
        args.requests,
    )
    .await?;

    display_comparison(&scenarios, &rari_results, &nextjs_results);
    calculate_summary(&scenarios, &rari_results, &nextjs_results);

    let results = BenchmarkResults {
        timestamp: time_utils::format_timestamp(SystemTime::now()),
        summary: TestSummary {
            test_requests: args.requests,
            warmup_requests: args.warmup,
            scenarios: scenarios.len(),
        },
        rari: rari_results,
        nextjs: nextjs_results,
    };

    save_results(&results, &args.results_dir).await?;

    println!("\n{}", "🎉 Benchmark completed!".green().bold());

    Ok(())
}
