#!/usr/bin/env node

import { execSync } from 'node:child_process'
import fs from 'node:fs/promises'
import path from 'node:path'
import { performance } from 'node:perf_hooks'
import process from 'node:process'
import pc from 'picocolors'

async function runBuild(name, directory, command) {
  console.warn(pc.bold(`\n🔨 Building ${name}...`))
  console.warn(pc.dim(`Directory: ${directory}`))
  console.warn(pc.dim(`Command: ${command}`))

  const startTime = performance.now()

  try {
    const result = execSync(command, {
      cwd: path.resolve(`${directory}`),
      env: { ...process.env, NODE_ENV: 'production' },
      encoding: 'utf8',
      timeout: 120000,
    })

    const endTime = performance.now()
    const duration = endTime - startTime

    console.warn(pc.green(`  ✅ ${name} built successfully in ${duration.toFixed(2)}ms`))

    return {
      success: true,
      duration,
      stdout: result,
      stderr: '',
      exitCode: 0,
    }
  }
  catch (error) {
    const endTime = performance.now()
    const duration = endTime - startTime

    console.warn(pc.red(`  ❌ ${name} build failed (exit code ${error.status || -1})`))
    console.warn(pc.red(`  Error: ${error.stderr || error.message}`))

    return {
      success: false,
      duration,
      stdout: error.stdout || '',
      stderr: error.stderr || error.message,
      exitCode: error.status || -1,
    }
  }
}

async function getClientBundleSize(name, directory) {
  let totalSize = 0
  let chunkCount = 0

  if (name === 'Next.js') {
    const staticDir = path.resolve(`${directory}/.next/static`)
    const chunksDir = path.join(staticDir, 'chunks')

    try {
      const chunkFiles = await fs.readdir(chunksDir, { recursive: true })
      for (const file of chunkFiles) {
        if (file.endsWith('.js')) {
          chunkCount++
          const filePath = path.join(chunksDir, file)
          try {
            const stats = await fs.stat(filePath)
            totalSize += stats.size
          }
          catch {
          }
        }
      }
    }
    catch {
    }
  }
  else if (name === 'Rari') {
    const distDir = path.resolve(`${directory}/dist`)
    try {
      const files = await fs.readdir(distDir, { recursive: true })
      for (const file of files) {
        if (file.endsWith('.js') || file.endsWith('.css')) {
          chunkCount++
          const filePath = path.join(distDir, file)
          try {
            const stats = await fs.stat(filePath)
            totalSize += stats.size
          }
          catch {
          }
        }
      }
    }
    catch {
    }
  }

  return { totalSize, chunkCount }
}

async function analyzeBuildOutput(name, result) {
  if (!result.success) {
    return { error: 'Build failed' }
  }

  const output = result.stdout + result.stderr

  const analysis = {
    duration: result.duration,
    bundleSize: null,
    chunkCount: null,
    warnings: 0,
    errors: 0,
  }

  analysis.warnings = (output.match(/warning/gi) || []).length
  analysis.errors = (output.match(/error/gi) || []).length

  const directory = name === 'Next.js' ? 'nextjs-app' : 'rari-app'
  const bundleInfo = await getClientBundleSize(name, directory)

  analysis.chunkCount = bundleInfo.chunkCount
  if (bundleInfo.totalSize > 0) {
    const sizeInKB = bundleInfo.totalSize / 1024
    analysis.bundleSize = `${sizeInKB.toFixed(2)} kB`
  }

  return analysis
}

function displayComparison(rariResult, nextjsResult) {
  console.warn(pc.bold('\n📊 Build Performance Comparison'))

  console.warn('\n⏱️  Build Times:')
  console.warn(`  🦀 Rari:     ${(rariResult.duration / 1000).toFixed(2)}s`)
  console.warn(`  🟢 Next.js:  ${(nextjsResult.duration / 1000).toFixed(2)}s`)

  const timeDiff = ((rariResult.duration - nextjsResult.duration) / nextjsResult.duration) * 100
  if (timeDiff < 0) {
    console.warn(pc.green(`  📈 Rari builds ${Math.abs(timeDiff).toFixed(1)}% faster`))
  }
  else {
    console.warn(pc.red(`  📉 Rari builds ${timeDiff.toFixed(1)}% slower`))
  }

  console.warn('\n📦 Bundle Information:')
  console.warn(`  🦀 Rari:`)
  console.warn(`     Size: ${rariResult.bundleSize || 'Unknown'}`)
  console.warn(`     Chunks: ${rariResult.chunkCount || 'Unknown'}`)
  console.warn(`     Warnings: ${rariResult.warnings}`)
  console.warn(`     Errors: ${rariResult.errors}`)

  console.warn(`  🟢 Next.js:`)
  console.warn(`     Size: ${nextjsResult.bundleSize || 'Unknown'}`)
  console.warn(`     Chunks: ${nextjsResult.chunkCount || 'Unknown'}`)
  console.warn(`     Warnings: ${nextjsResult.warnings}`)
  console.warn(`     Errors: ${nextjsResult.errors}`)
}

async function saveResults(rariResult, nextjsResult) {
  const timestamp = new Date().toISOString()
  const results = {
    timestamp,
    rari: rariResult,
    nextjs: nextjsResult,
  }

  try {
    await fs.mkdir('results', { recursive: true })
  }
  catch {
  }

  await fs.writeFile(
    `./results/buildtimes-${timestamp.slice(0, 10)}.json`,
    JSON.stringify(results, null, 2),
  )

  console.warn(pc.dim(`\n💾 Results saved to ./results/buildtimes-${timestamp.slice(0, 10)}.json`))
}

async function main() {
  console.warn(pc.bold(pc.cyan('🔨 Rari vs Next.js Build Time Comparison')))
  console.warn(pc.dim('This benchmark compares build performance and bundle analysis\n'))

  console.warn(pc.yellow('⚠️  This will run production builds which may take some time'))
  console.warn(pc.dim('Starting build comparison in 3 seconds...'))
  await new Promise(resolve => setTimeout(resolve, 3000))

  const rariResult = await runBuild('Rari', 'rari-app', 'pnpm run build')
  const rariAnalysis = await analyzeBuildOutput('Rari', rariResult)

  const nextjsResult = await runBuild('Next.js', 'nextjs-app', 'pnpm run build')
  const nextjsAnalysis = await analyzeBuildOutput('Next.js', nextjsResult)

  displayComparison(rariAnalysis, nextjsAnalysis)
  await saveResults(rariAnalysis, nextjsAnalysis)

  console.warn(pc.bold(pc.green('\n🎉 Build comparison completed!')))
}

main().catch(console.error)
