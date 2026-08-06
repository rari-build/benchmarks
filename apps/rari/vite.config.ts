import path from 'node:path'
import tailwindcss from '@tailwindcss/vite'
import { rari } from 'rari/vite'
import { defineConfig } from 'vite-plus'
import { fmt, lint } from '../../.config/lint'

export default defineConfig({
  plugins: [rari(), tailwindcss()],
  resolve: {
    alias: {
      '@': path.resolve(import.meta.dirname, 'src'),
      '@benchmark/shared': path.resolve(import.meta.dirname, '../../shared/src'),
    },
  },
  fmt,
  lint,
})
