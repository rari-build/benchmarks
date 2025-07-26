#!/usr/bin/env node

import fs from 'node:fs/promises'
import process from 'node:process'
import autocannon from 'autocannon'
import pc from 'picocolors'

const MODE = process.env.BENCHMARK_MODE || 'development'
const IS_PRODUCTION = MODE === 'production'
const RARI_PORT = IS_PRODUCTION ? 3000 : 5173
const NEXTJS_PORT = IS_PRODUCTION ? 3001 : 3000

const LOAD_TEST_CONFIG = {
  duration: 30,
  connections: 50,
  pipelining: 1,
  timeout: 10,
}

async function runLoadTest(name, port, path = '/') {
  const url = `http://localhost:${port}${path}`

  console.warn(pc.bold(`\n🔥 Load Testing ${name}`))
  console.warn(pc.dim(`URL: ${url}`))
  console.warn(pc.dim(`Duration: ${LOAD_TEST_CONFIG.duration}s, Connections: ${LOAD_TEST_CONFIG.connections}`))

  return new Promise((resolve, reject) => {
    const instance = autocannon({
      url,
      duration: LOAD_TEST_CONFIG.duration,
      connections: LOAD_TEST_CONFIG.connections,
      pipelining: LOAD_TEST_CONFIG.pipelining,
      timeout: LOAD_TEST_CONFIG.timeout,
    }, (err, result) => {
      if (err) {
        reject(err)
      }
      else {
        resolve(result)
      }
    })

    autocannon.track(instance, {
      renderProgressBar: true,
      renderResultsTable: false,
      renderLatencyTable: false,
    })
  })
}

function formatResults(result) {
  return {
    requests: {
      total: result.requests.total,
      average: result.requests.average,
      mean: result.requests.mean,
      stddev: result.requests.stddev,
      min: result.requests.min,
      max: result.requests.max,
    },
    latency: {
      average: result.latency.average,
      mean: result.latency.mean,
      stddev: result.latency.stddev,
      min: result.latency.min,
      max: result.latency.max,
      p50: result.latency.p50,
      p90: result.latency.p90,
      p95: result.latency.p95,
      p99: result.latency.p99,
    },
    throughput: {
      average: result.throughput.average,
      mean: result.throughput.mean,
      stddev: result.throughput.stddev,
      min: result.throughput.min,
      max: result.throughput.max,
    },
    errors: result.errors,
    timeouts: result.timeouts,
    duration: result.duration,
    start: result.start,
    finish: result.finish,
  }
}

function displayComparison(rariResult, nextjsResult) {
  console.warn(pc.bold('\n📊 Load Test Comparison'))

  const rari = formatResults(rariResult)
  const nextjs = formatResults(nextjsResult)

  console.warn('\n📈 Throughput (req/sec):')
  console.warn(`  🦀 Rari:     ${rari.requests.average.toFixed(2)} (±${rari.requests.stddev.toFixed(2)})`)
  console.warn(`  🟢 Next.js:  ${nextjs.requests.average.toFixed(2)} (±${nextjs.requests.stddev.toFixed(2)})`)

  const throughputDiff = ((rari.requests.average - nextjs.requests.average) / nextjs.requests.average) * 100
  if (throughputDiff > 0) {
    console.warn(pc.green(`  📈 Rari handles ${throughputDiff.toFixed(1)}% more requests/sec`))
  }
  else {
    console.warn(pc.red(`  📉 Rari handles ${Math.abs(throughputDiff).toFixed(1)}% fewer requests/sec`))
  }

  console.warn('\n⏱️  Latency (ms):')

  const rariAvg = rari.latency?.average || 0
  const rariP95 = rari.latency?.p95 || 0
  const nextjsAvg = nextjs.latency?.average || 0
  const nextjsP95 = nextjs.latency?.p95 || 0

  console.warn(`  🦀 Rari:     ${rariAvg.toFixed(2)}ms (P95: ${rariP95.toFixed(2)}ms)`)
  console.warn(`  🟢 Next.js:  ${nextjsAvg.toFixed(2)}ms (P95: ${nextjsP95.toFixed(2)}ms)`)

  const latencyDiff = nextjsAvg > 0 ? ((rariAvg - nextjsAvg) / nextjsAvg) * 100 : 0
  if (latencyDiff < 0) {
    console.warn(pc.green(`  📈 Rari is ${Math.abs(latencyDiff).toFixed(1)}% faster response time`))
  }
  else {
    console.warn(pc.red(`  📉 Rari is ${latencyDiff.toFixed(1)}% slower response time`))
  }

  console.warn('\n🚨 Errors:')
  console.warn(`  🦀 Rari:     ${rari.errors} errors, ${rari.timeouts} timeouts`)
  console.warn(`  🟢 Next.js:  ${nextjs.errors} errors, ${nextjs.timeouts} timeouts`)
}

async function saveResults(rariResult, nextjsResult) {
  const timestamp = new Date().toISOString()
  const results = {
    timestamp,
    config: LOAD_TEST_CONFIG,
    rari: formatResults(rariResult),
    nextjs: formatResults(nextjsResult),
  }

  try {
    await fs.mkdir('./results', { recursive: true })
  }
  catch {
  }

  await fs.writeFile(
    `./results/loadtest-${timestamp.slice(0, 10)}.json`,
    JSON.stringify(results, null, 2),
  )

  console.warn(pc.dim(`\n💾 Results saved to ./results/loadtest-${timestamp.slice(0, 10)}.json`))
}

async function main() {
  console.warn(pc.bold(pc.cyan('🔥 Rari vs Next.js Load Test')))
  console.warn(pc.dim('This test measures concurrent request handling performance\n'))

  console.warn(pc.blue(`📊 Running in ${pc.bold(MODE.toUpperCase())} mode`))
  console.warn('')

  // Test connectivity
  try {
    await fetch(`http://localhost:${RARI_PORT}`)
    console.warn(pc.green('✅ Rari server is responding'))
  }
  catch {
    const command = IS_PRODUCTION ? 'pnpm start' : 'pnpm dev'
    console.warn(pc.red(`❌ Rari server is not responding. Please start it with: cd rari-app && ${command}`))
    process.exit(1)
  }

  try {
    await fetch(`http://localhost:${NEXTJS_PORT}`)
    console.warn(pc.green('✅ Next.js server is responding'))
  }
  catch {
    const command = IS_PRODUCTION ? 'PORT=3001 pnpm start' : 'pnpm dev'
    console.warn(pc.red(`❌ Next.js server is not responding. Please start it with: cd nextjs-app && ${command}`))
    process.exit(1)
  }

  console.warn(pc.yellow('\n⚠️  This test will generate significant load on both servers'))
  console.warn(pc.dim('Starting load test in 3 seconds...'))
  await new Promise(resolve => setTimeout(resolve, 3000))

  const rariResult = await runLoadTest('Rari', RARI_PORT)
  await new Promise(resolve => setTimeout(resolve, 2000)) // Brief pause between tests

  const nextjsResult = await runLoadTest('Next.js', NEXTJS_PORT)

  displayComparison(rariResult, nextjsResult)
  await saveResults(rariResult, nextjsResult)

  console.warn(pc.bold(pc.green('\n🎉 Load test completed!')))
}

main().catch(console.error)
