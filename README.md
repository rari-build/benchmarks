<a href="https://rari.build" target="_blank">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset=".github/assets/rari-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset=".github/assets/rari-light.svg">
    <img alt="rari" src=".github/assets/rari-light.svg" width="200">
  </picture>
</a>

> Runtime Accelerated Rendering Infrastructure

[![npm version](https://img.shields.io/npm/v/rari.svg)](https://www.npmjs.com/package/rari)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Discord](https://img.shields.io/badge/chat-discord-blue?style=flat&logo=discord)](https://discord.gg/GSh2Ak3b8Q)

# rari vs Next.js Benchmark Suite

This benchmark suite provides a comprehensive comparison between **rari** (Rust-powered React Server Components) and **Next.js** (Node.js-based React framework).

## Benchmark Objectives

### Performance Metrics
- **Server-side rendering speed** - Time to render all components on server
- **Time to First Byte (TTFB)** - Server response latency (P50, P95, P99)
- **Bundle size comparison** - Client-side JavaScript payload
- **Build times** - Production build speeds
- **Concurrent request handling** - Throughput under load (requests/sec)
- **Memory usage** - Runtime memory consumption under load

### What's Being Tested
The benchmark suite tests a single comprehensive homepage route (`/`) that includes:
- **8 Server Components** - Counter, TestComponent, ShoppingList, WhatsHot, EnvTestComponent, FetchExample, ServerWithClient, and Markdown
- **Static rendering** - All components rendered server-side
- **Real-world complexity** - Mix of simple and complex components with various rendering patterns

## Quick Start

### Prerequisites
- Node.js 22+
- pnpm (install with `corepack enable`)
- Rust and Cargo
- **just** - Command runner (install with `cargo install just`)

### Setup
```bash
# One-command setup (installs all dependencies and tools)
just setup

# Or complete initialization (setup + build + compile)
just init
```

### Run Benchmarks
```bash
# Build both apps for production
just build

# Run complete benchmark suite
just benchmark-all

# Run specific benchmarks
just buildtest
just benchmark
just loadtest

# Quick tests with oha
just quick-test-rari
just quick-test-nextjs
```

## Test Scenarios

### 1. Performance Benchmark
Tests server-side rendering performance with sequential requests:
- **Warmup phase** - 50 requests to warm up the server
- **Test phase** - 20 measured requests
- **Metrics collected** - Min, max, avg, P50, P95, P99 response times, response size, error rate

### 2. Load Test
Tests concurrent request handling using `oha`:
- **Duration** - 30 seconds (configurable)
- **Concurrent connections** - 50 (configurable)
- **Metrics collected** - Throughput (req/sec), latency percentiles, error rates, timeouts

### 3. Build Time Test
Compares production build performance:
- **Build command** - `pnpm run build` for both frameworks
- **Metrics collected** - Build duration, bundle size, chunk count, warnings, errors

## Metrics Collection

### Performance Benchmark Metrics
- **Response times** - Min, max, avg, P50, P95, P99 (in milliseconds)
- **Response size** - Average payload size in bytes
- **Success rate** - Percentage of successful requests
- **Error count** - Number of failed requests

### Load Test Metrics
- **Throughput** - Requests per second (avg, min, max)
- **Latency** - Response time percentiles (P50, P90, P95, P99) in milliseconds
- **Errors** - Failed requests and timeouts
- **Duration** - Total test duration

### Build Metrics
- **Build time** - Total production build duration
- **Bundle size** - Total size of client-side JavaScript and CSS
- **Chunk count** - Number of generated files
- **Warnings/Errors** - Build output analysis

## Configuration

Both applications are configured to be as equivalent as possible:

### Shared Features
- App Router with file-based routing
- Single homepage route (`/`) with 8 server components
- Server components by default
- TypeScript throughout
- Tailwind CSS for styling
- Identical component implementations

### rari App
- Rust-powered React Server Components runtime
- Vite-based build system
- Production server on port 3000

### Next.js App
- Node.js-based React framework
- Turbopack build system
- Production server on port 3001

## Actual Results

Based on the latest benchmark run (January 26, 2026):

### Build Performance

| Metric | rari | Next.js | Improvement |
|--------|------|---------|-------------|
| Build Time | 1.41s | 4.10s | 65.6% faster |
| Client Bundle | 266.04 KB (2 files) | 564.84 KB (8 files) | 52.9% smaller |

### Performance Benchmark

| Metric | rari | Next.js | Improvement |
|--------|------|---------|-------------|
| Average Response Time | 0.35ms | 2.64ms | 86.6% faster |
| P95 Latency | 0.38ms | 3.44ms | 89.0% faster |
| Response Size | 33,389 bytes | 84,317 bytes | 60.4% smaller |

### Load Test Performance (30s duration, 50 concurrent connections)

| Metric | rari | Next.js | Improvement |
|--------|------|---------|-------------|
| Throughput | 14,085.06 req/sec | 1,624.41 req/sec | 767.1% higher |
| Average Latency | 3.55ms | 30.79ms | 88.5% faster |
| P95 Latency | 5.78ms | 38.43ms | 84.9% faster |
| Total Requests | 422,604 | 48,736 | 767.1% more |
| Errors | 0 | 0 | 100% success rate |
| Timeouts | 0 | 0 | 100% success rate |

## Running Individual Tests

### Starting Servers
```bash
# Start Rari production server (port 3000)
just start-rari

# Start Next.js production server (port 3001)
just start-nextjs

# Check if servers are running
just check-servers
```

### Performance Testing
```bash
# Run performance benchmark (requires servers to be running)
just benchmark
```

### Load Testing
```bash
# Run load test (requires servers to be running)
just loadtest

# Quick load tests with custom parameters
just quick-test-rari 30s 100
just quick-test-nextjs 30s 100
```

### Build Time Testing
```bash
# Run build time comparison
just buildtest
```

### Viewing Results
```bash
# View latest results
just results

# View specific result types
just results-build
just results-perf
just results-load
```

### Development Commands
```bash
# Clean build artifacts
just clean

# Clean and rebuild
just rebuild

# Format and check code
just fmt
just check

# Compile benchmark tools
just compile
```

## Available Commands

Run `just` to see all available commands, or `just --list` for a detailed list.

### Common Workflows

**First-time setup:**
```bash
just init
```

**Running benchmarks:**
```bash
# 1. Build the apps
just build

# 2. Start servers (in separate terminals)
just start-rari
just start-nextjs

# 3. Run benchmarks
just benchmark-all
```

**Quick iteration:**
```bash
just rebuild
just buildtest
just check-servers
```

## Contributing

When adding new benchmark scenarios:

1. **Ensure parity** - Both apps should have equivalent functionality
2. **Update both apps** - Add components/routes to both rari-app and nextjs-app
3. **Update benchmark tools** - Modify the Rust benchmark tools in `tools/benchmark/src/`
4. **Document changes** - Update this README with new test scenarios
5. **Run all tests** - Verify with `just benchmark-all`

## License

MIT License - see [LICENSE](LICENSE) for details.
