import path from 'node:path'
import tailwindcss from '@tailwindcss/vite'
import { rari } from 'rari/vite'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [
    rari({
      rateLimit: {
        enabled: false,
      },
      spamBlocker: {
        enabled: false,
      },
    }),
    tailwindcss(),
  ],
  resolve: {
    alias: {
      '@': path.resolve(import.meta.dirname, 'src'),
      '@benchmark/shared': path.resolve(import.meta.dirname, '../../shared/src'),
    },
  },
})
