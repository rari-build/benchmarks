set windows-shell := ["powershell"]
set shell := ["bash", "-cu"]

# List all available commands
_default:
    just --list -u

# --- Setup commands ---

# Setup the project (Node.js + Rust tools)
setup:
    just check-prerequisites
    pnpm install
    cargo install oha
    @echo "✅ Setup complete!"

# Check if all prerequisites are installed
check-prerequisites:
    @command -v cargo >/dev/null 2>&1 || { echo "❌ Cargo is not installed. Please install Rust from https://rustup.rs/"; exit 1; }
    @command -v node >/dev/null 2>&1 || { echo "❌ Node.js is not installed. Please install Node.js from https://nodejs.org/"; exit 1; }
    @command -v pnpm >/dev/null 2>&1 || { echo "❌ pnpm is not installed. Run 'corepack enable' to install it."; exit 1; }
    @echo "✅ All prerequisites are installed"

# --- Build commands ---

# Build both apps for production
build: build-rari build-nextjs

# Build rari app
build-rari:
    cd rari-app && pnpm run build

# Build Next.js app
build-nextjs:
    cd nextjs-app && pnpm run build

# Compile benchmark tools
compile:
    cargo build --manifest-path ./tools/benchmark/Cargo.toml --release

# --- Benchmark commands ---

# Run build time benchmark
buildtest:
    cargo run --manifest-path ./tools/benchmark/Cargo.toml --release --bin build-times

# Start rari production server (port 3000)
start-rari:
    cd rari-app && pnpm run start

# Start Next.js production server (port 3001)
start-nextjs:
    cd nextjs-app && pnpm run start

# Check if servers are running
check-servers:
    @curl -s http://localhost:3000 > /dev/null && echo "✅ rari is running on port 3000" || echo "❌ rari is not running on port 3000"
    @curl -s http://localhost:3001 > /dev/null && echo "✅ Next.js is running on port 3001" || echo "❌ Next.js is not running on port 3001"

# --- Quick test commands ---

# Run performance benchmark (requires servers to be running)
benchmark:
    cargo run --manifest-path ./tools/benchmark/Cargo.toml --release --bin performance

# Run load test benchmark (requires servers to be running)
loadtest:
    cargo run --manifest-path ./tools/benchmark/Cargo.toml --release --bin load-test

# Run all benchmarks (requires servers to be running)
benchmark-all:
    just buildtest
    just benchmark
    just loadtest

# --- Server commands ---

# Quick test with oha directly (rari)
quick-test-rari duration="10s" connections="50":
    oha http://localhost:3000 -z {{duration}} -c {{connections}} --no-tui

# Quick test with oha directly (Next.js)
quick-test-nextjs duration="10s" connections="50":
    oha http://localhost:3001 -z {{duration}} -c {{connections}} --no-tui

# --- Results commands ---

# View latest benchmark results
results:
    @echo "📊 Latest benchmark results:"
    @echo ""
    @if [ -f results/latest.json ]; then \
        cat results/latest.json | jq .; \
    else \
        echo "No results found. Run benchmarks first."; \
    fi

# View build time results
results-build:
    @ls -t results/buildtimes-*.json | head -1 | xargs cat | jq .

# View performance results
results-perf:
    @ls -t results/performance-*.json | head -1 | xargs cat | jq .

# View load test results
results-load:
    @ls -t results/loadtest-*.json | head -1 | xargs cat | jq .

# --- Clean commands ---

# Clean build artifacts
clean:
    rm -rf rari-app/dist
    rm -rf nextjs-app/.next
    rm -rf target

# Clean and rebuild everything
rebuild: clean build

# --- Development commands ---

# Format code
fmt:
    cargo fmt --manifest-path ./tools/benchmark/Cargo.toml

# Check code
check:
    cargo check --manifest-path ./tools/benchmark/Cargo.toml
    cargo clippy --manifest-path ./tools/benchmark/Cargo.toml

# --- Combined workflow commands ---

# Full workflow: setup, build, and compile tools
init: setup build compile
    @echo "✅ Project initialized and ready for benchmarking!"
