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
│   │   ├── components/
│   │   ├── pages/
│   │   └── functions/
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
cd ../scripts && pnpm install
```

### Run Benchmarks
```bash
# Run complete benchmark suite
cd benchmarks/scripts
pnpm run benchmark:all

# Run specific benchmarks
pnpm run benchmark:performance  # Server performance
pnpm run benchmark:build       # Build times
pnpm run benchmark:memory      # Memory usage
pnpm run benchmark:load        # Load testing
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
- File-based routing with dynamic routes
- Server components with data fetching
- Client components for interactivity
- TypeScript throughout
- Tailwind CSS for styling
- Error boundaries and loading states

### Next.js App Features
- App Router with equivalent route structure
- Server Components and Client Components
- Same data fetching patterns
- Identical TypeScript configuration
- Same Tailwind CSS setup
- Equivalent error handling

## Expected Results

Based on Rari's architecture, we expect to see:

### Performance Advantages
- **2-4x faster** server component rendering
- **Lower memory usage** due to Rust runtime efficiency
- **Better concurrent handling** with async Rust performance
- **Faster cold starts** with optimized runtime

### Trade-offs
- **Ecosystem maturity** - Next.js has broader ecosystem
- **Development tooling** - Next.js has more mature dev tools
- **Community support** - Next.js has larger community
- **Production stability** - Next.js has longer production track record

## Running Individual Tests

### Performance Testing
```bash
# Start applications
cd rari-app && pnpm dev &
cd nextjs-app && pnpm dev &

# Run performance comparison
cd scripts && pnpm run test:performance
```

### Load Testing
```bash
# Production builds
cd rari-app && pnpm build && pnpm start &
cd nextjs-app && pnpm build && pnpm start &

# Load test
cd scripts && pnpm run test:load
```

### Memory Profiling
```bash
cd scripts && pnpm run test:memory
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
