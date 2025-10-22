# Rari vs Next.js Benchmark Suite

This benchmark suite provides a comprehensive comparison between **Rari** (Rust-powered React Server Components) and **Next.js** (Node.js-based React framework).

## Benchmark Objectives

### Performance Metrics
- **Server-side rendering speed** - Time to render components on server
- **Time to First Byte (TTFB)** - Server response latency
- **Bundle size comparison** - Client-side JavaScript payload
- **Memory usage** - Runtime memory consumption
- **Concurrent request handling** - Throughput under load
- **Build times** - Development and production build speeds
- **Cold start performance** - Initial server startup time

### Feature Parity Testing
- **Server Components** - RSC rendering capabilities
- **File-based routing** - Route generation and navigation
- **Data fetching** - Server-side data loading patterns
- **Streaming** - Progressive content delivery
- **Error handling** - Error boundaries and recovery
- **TypeScript support** - Type safety and DX

## Structure

```
benchmarks/
├── README.md                 # This file - benchmark overview
├── shared/                   # Shared test data and utilities
│   ├── components/          # Common components for both frameworks
│   ├── data/               # Test datasets and API responses
│   └── utils/              # Benchmark utilities and helpers
├── rari-app/               # Rari test application
│   ├── src/
│   │   ├── app/            # App Router structure
│   │   ├── components/
│   │   └── actions/
│   ├── package.json
│   └── vite.config.ts
├── nextjs-app/             # Next.js equivalent application
│   ├── src/
│   │   ├── components/
│   │   ├── app/            # App Router structure
│   │   └── lib/
│   ├── package.json
│   └── next.config.js
├── scripts/                # Benchmark execution scripts
│   ├── performance.js      # Performance measurement suite
│   ├── load-test.js       # Concurrent load testing
│   ├── build-times.js     # Build performance comparison
│   └── memory-usage.js    # Memory profiling
└── results/               # Benchmark results and reports
    ├── latest.json        # Most recent benchmark results
    └── history/          # Historical benchmark data
```

## Quick Start

### Prerequisites
- Node.js 22+
- pnpm (recommended)
- Rust (for Rari development builds)

### Setup
```bash
# Install dependencies for both apps
cd benchmarks/rari-app && pnpm install
cd ../nextjs-app && pnpm install

# Install benchmark scripts dependencies
cd .. && pnpm install
```

### Run Benchmarks
```bash
# From benchmarks/ directory

# Run complete benchmark suite (development mode)
pnpm run benchmark:all:dev

# Run complete benchmark suite (production mode)
pnpm run benchmark:all:prod

# Run specific benchmarks
pnpm run benchmark:dev      # Server performance (dev mode)
pnpm run benchmark:prod     # Server performance (prod mode)
pnpm run loadtest:dev       # Load testing (dev mode)
pnpm run loadtest:prod      # Load testing (prod mode)
pnpm run buildtest          # Build times comparison
```

## Test Scenarios

### 1. Server Component Rendering
- **Simple components** - Basic text and props
- **Data fetching components** - API calls and database queries
- **Complex layouts** - Nested components with multiple data sources
- **Markdown processing** - Server-side content transformation
- **Image optimization** - Server-side image processing

### 2. Routing Performance
- **Static routes** - Simple page navigation
- **Dynamic routes** - Parameterized paths ([id], [slug])
- **Nested routes** - Multi-level route hierarchies
- **Catch-all routes** - [...slug] patterns
- **Route transitions** - Client-side navigation speed

### 3. Concurrent Load Testing
- **1-10 concurrent users** - Light load
- **50-100 concurrent users** - Medium load
- **200+ concurrent users** - Heavy load
- **Stress testing** - Resource exhaustion scenarios

### 4. Real-world Scenarios
- **Blog application** - Static content with dynamic routing
- **E-commerce catalog** - Product listings with search/filter
- **Dashboard** - Real-time data with charts and tables
- **Content management** - CRUD operations with server validation

## Metrics Collection

### Performance Metrics
- **Response time** - P50, P95, P99 percentiles
- **Throughput** - Requests per second
- **Error rate** - Failed request percentage
- **Resource usage** - CPU, memory, network

### Development Experience
- **Build times** - Development and production builds
- **Hot reload speed** - Time for changes to reflect
- **Bundle analysis** - JavaScript payload sizes
- **TypeScript compilation** - Type checking performance

### Quality Metrics
- **First Contentful Paint (FCP)**
- **Largest Contentful Paint (LCP)**
- **Cumulative Layout Shift (CLS)**
- **Time to Interactive (TTI)**

## Configuration

Both applications are configured to be as equivalent as possible:

### Rari App Features
- App Router with file-based routing and dynamic routes
- Server components by default with data fetching
- Client components for interactivity ('use client')
- Server actions for mutations ('use server')
- TypeScript throughout
- Tailwind CSS for styling
- Error boundaries and loading states

### Next.js App Features
- App Router with equivalent route structure
- Server Components and Client Components
- Server Actions for mutations
- Same data fetching patterns
- Identical TypeScript configuration
- Same Tailwind CSS setup
- Equivalent error handling

## Actual Results

Based on our benchmarks, Rari delivers:

### Performance Advantages
- **3.8x faster** response times (0.69ms vs 2.58ms avg)
- **10.5x higher** throughput (20,226 vs 1,934 req/sec)
- **12x faster** P99 latency under load (4ms vs 48ms)
- **68% smaller** client bundles (27.6 KB vs 85.9 KB)
- **5.6x faster** builds (1.64s vs 9.11s)

### Trade-offs
- **Ecosystem maturity** - Next.js has broader ecosystem
- **Development tooling** - Next.js has more mature dev tools
- **Community support** - Next.js has larger community
- **Production stability** - Next.js has longer production track record

## Running Individual Tests

### Performance Testing
```bash
# From benchmarks/ directory

# Development mode (apps will be started automatically)
pnpm run benchmark:dev

# Production mode (apps will be built and started automatically)
pnpm run benchmark:prod
```

### Load Testing
```bash
# From benchmarks/ directory

# Development mode
pnpm run loadtest:dev

# Production mode (recommended for accurate results)
pnpm run loadtest:prod
```

### Build Time Testing
```bash
# From benchmarks/ directory
pnpm run buildtest
```

## Contributing

When adding new benchmark scenarios:

1. **Ensure parity** - Both apps should have equivalent functionality
2. **Document metrics** - Clearly define what's being measured
3. **Reproducible tests** - Include setup and teardown procedures
4. **Realistic scenarios** - Test real-world usage patterns
5. **Update documentation** - Keep README and scripts current

## License

This benchmark suite is part of the Rari project and follows the same MIT license.
