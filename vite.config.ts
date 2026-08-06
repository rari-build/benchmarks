import { defineConfig } from 'vite-plus'
import { fmt, lint } from './.config/lint'

export default defineConfig({
  fmt,
  lint,
})
