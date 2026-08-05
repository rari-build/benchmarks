import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  // Next 16.3 defaults to CLI typechecking (`typescript/bin/tsc`).
  // This workspace aliases `typescript` → `@typescript/typescript6` (tsc6 + API)
  // per https://devblogs.microsoft.com/typescript/announcing-typescript-7-0/#running-side-by-side-with-typescript-6.0
  // so use the compiler API instead of the missing `tsc` binary.
  experimental: {
    useTypeScriptCli: false,
  },
}

export default nextConfig
