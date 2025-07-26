#!/usr/bin/env node

import fs from 'node:fs/promises'
import { performance } from 'node:perf_hooks'
import process from 'node:process'
import Table from 'cli-table3'
import pc from 'picocolors'

const MODE = process.env.BENCHMARK_MODE || 'development'
const IS_PRODUCTION = MODE === 'production'
const RARI_PORT = IS_PRODUCTION ? 3000 : 5173
const NEXTJS_PORT = IS_PRODUCTION ? 3001 : 3000
const WARMUP_REQUESTS = 5
const TEST_REQUESTS = 10

const SCENARIOS = [
  { path: '/', name: 'Homepage (All Components)' },
]

async function measureRequest(url, requests = TEST_REQUESTS) {
  const times = []
  const sizes = []
  let errors = 0

  for (let i = 0; i < WARMUP_REQUESTS; i++) {
    try {
      await fetch(url)
    }
    catch {
    }
  }

  console.warn(`  Testing ${url}...`)

  for (let i = 0; i < requests; i++) {
    const start = performance.now()

    try {
      const response = await fetch(url)
      const end = performance.now()

      if (response.ok) {
        const text = await response.text()
        times.push(end - start)
        sizes.push(text.length)
      }
      else {
        errors++
      }
    }
    catch {
      errors++
    }

    await new Promise(resolve => setTimeout(resolve, 50))
  }

  if (times.length === 0) {
    return null
  }

  const sortedTimes = times.sort((a, b) => a - b)
  const avgSize = sizes.reduce((a, b) => a + b, 0) / sizes.length

  return {
    min: Math.min(...times),
    max: Math.max(...times),
    avg: times.reduce((a, b) => a + b, 0) / times.length,
    p50: sortedTimes[Math.floor(sortedTimes.length * 0.5)],
    p95: sortedTimes[Math.floor(sortedTimes.length * 0.95)],
    p99: sortedTimes[Math.floor(sortedTimes.length * 0.99)],
    avgSize: Math.round(avgSize),
    errors,
    successRate: ((requests - errors) / requests) * 100,
  }
}

async function benchmarkFramework(name, port, color) {
  console.warn(pc[color](`\n🚀 Benchmarking ${name} (port ${port})`))

  const results = {}

  for (const scenario of SCENARIOS) {
    const url = `http://localhost:${port}${scenario.path}`
    console.warn(`\n📊 ${scenario.name}`)

    const result = await measureRequest(url)
    if (result) {
      results[scenario.name] = result
      console.warn(`  ✅ Avg: ${result.avg.toFixed(2)}ms, P95: ${result.p95.toFixed(2)}ms, Size: ${result.avgSize}b`)
    }
    else {
      results[scenario.name] = { error: 'Failed to get valid responses' }
      console.warn(`  ❌ Failed to get valid responses`)
    }
  }

  return results
}

function displayComparison(rariResults, nextjsResults) {
  console.warn(pc.bold('\n📈 Performance Comparison'))

  const table = new Table({
    head: ['Scenario', 'Rari (ms)', 'Next.js (ms)', 'Difference', 'Winner'],
    colWidths: [20, 15, 15, 15, 10],
  })

  for (const scenario of SCENARIOS) {
    const rari = rariResults[scenario.name]
    const nextjs = nextjsResults[scenario.name]

    if (rari?.avg && nextjs?.avg) {
      const diff = ((rari.avg - nextjs.avg) / nextjs.avg) * 100
      const winner = rari.avg < nextjs.avg ? '🦀 Rari' : '🟢 Next.js'
      const diffStr = diff > 0 ? `+${diff.toFixed(1)}%` : `${diff.toFixed(1)}%`
      const diffColor = diff < 0 ? pc.green : pc.red

      table.push([
        scenario.name,
        rari.avg.toFixed(2),
        nextjs.avg.toFixed(2),
        diffColor(diffStr),
        winner,
      ])
    }
    else {
      table.push([
        scenario.name,
        rari?.error ? 'Error' : 'N/A',
        nextjs?.error ? 'Error' : 'N/A',
        'N/A',
        'N/A',
      ])
    }
  }

  console.warn(table.toString())
}

async function calculateSummary(rariResults, nextjsResults) {
  const validScenarios = SCENARIOS.filter(s =>
    rariResults[s.name]?.avg && nextjsResults[s.name]?.avg,
  )

  if (validScenarios.length === 0) {
    console.warn(pc.red('\n❌ No valid results to compare'))
    return
  }

  const rariAvg = validScenarios.reduce((sum, s) =>
    sum + rariResults[s.name].avg, 0) / validScenarios.length
  const nextjsAvg = validScenarios.reduce((sum, s) =>
    sum + nextjsResults[s.name].avg, 0) / validScenarios.length

  const improvement = ((nextjsAvg - rariAvg) / nextjsAvg) * 100

  console.warn(pc.bold('\n📊 Summary'))
  console.warn(`Average Response Time:`)
  console.warn(`  🦀 Rari:     ${rariAvg.toFixed(2)}ms`)
  console.warn(`  🟢 Next.js:  ${nextjsAvg.toFixed(2)}ms`)

  if (improvement > 0) {
    console.warn(pc.green(`  📈 Rari is ${improvement.toFixed(1)}% faster`))
  }
  else {
    console.warn(pc.red(`  📉 Rari is ${Math.abs(improvement).toFixed(1)}% slower`))
  }
}

async function saveResults(rariResults, nextjsResults) {
  const timestamp = new Date().toISOString()
  const results = {
    timestamp,
    rari: rariResults,
    nextjs: nextjsResults,
    summary: {
      testRequests: TEST_REQUESTS,
      warmupRequests: WARMUP_REQUESTS,
      scenarios: SCENARIOS.length,
    },
  }

  try {
    await fs.mkdir('./results', { recursive: true })
  }
  catch {
  }

  await fs.writeFile(
    `./results/performance-${timestamp.slice(0, 10)}.json`,
    JSON.stringify(results, null, 2),
  )

  await fs.writeFile(
    './results/latest.json',
    JSON.stringify(results, null, 2),
  )

  console.warn(pc.dim(`\n💾 Results saved to ./results/performance-${timestamp.slice(0, 10)}.json`))
}

async function main() {
  console.warn(pc.bold(pc.cyan('🏁 Rari vs Next.js Performance Benchmark')))
  console.warn(pc.dim('This benchmark compares server-side rendering performance\n'))

  console.warn(pc.blue(`📊 Running in ${pc.bold(MODE.toUpperCase())} mode`))
  console.warn('')

  console.warn(pc.yellow('⚠️  Make sure both applications are running:'))
  if (IS_PRODUCTION) {
    console.warn(`  🦀 Rari:     http://localhost:${RARI_PORT} (run: pnpm start)`)
    console.warn(`  🟢 Next.js:  http://localhost:${NEXTJS_PORT} (run: PORT=3001 pnpm start)`)
  }
  else {
    console.warn(`  🦀 Rari:     http://localhost:${RARI_PORT} (run: pnpm dev)`)
    console.warn(`  🟢 Next.js:  http://localhost:${NEXTJS_PORT} (run: pnpm dev)`)
  }
  console.warn('')

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
    const command = IS_PRODUCTION ? 'pnpm start' : 'pnpm dev'
    console.warn(pc.red(`❌ Next.js server is not responding. Please start it with: cd nextjs-app && ${command}`))
    process.exit(1)
  }

  console.warn(pc.dim('\nStarting benchmark in 3 seconds...'))
  await new Promise(resolve => setTimeout(resolve, 3000))

  const rariResults = await benchmarkFramework('Rari', RARI_PORT, 'yellow')
  const nextjsResults = await benchmarkFramework('Next.js', NEXTJS_PORT, 'green')

  displayComparison(rariResults, nextjsResults)
  await calculateSummary(rariResults, nextjsResults)
  await saveResults(rariResults, nextjsResults)

  console.warn(pc.bold(pc.green('\n🎉 Benchmark completed!')))
}

main().catch(console.error)
