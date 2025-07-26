import tailwindcss from '@tailwindcss/vite'
import react from '@vitejs/plugin-react-oxc'
import { rari } from 'rari'
import { defineConfig } from 'rolldown-vite'

export default defineConfig({
  plugins: [rari(), react(), tailwindcss()],
  build: {
    rollupOptions: {
      input: {
        main: './index.html',
      },
      external: ['react', 'react-dom', 'react/jsx-runtime', 'react/jsx-dev-runtime'],
    },
  },
})
