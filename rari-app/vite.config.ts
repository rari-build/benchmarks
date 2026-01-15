import tailwindcss from '@tailwindcss/vite'
import { rari } from 'rari/vite'
import { defineConfig } from 'rolldown-vite'

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
})
