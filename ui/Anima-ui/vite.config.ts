import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import path from 'path';

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '$AnimaCanvas': path.resolve(__dirname, '../../canvas/Anima-canvas/pkg'),
      '$AnimaEngine': path.resolve(__dirname, '../../engine/Anima-engine/pkg')
    }
  },
  server: {
    fs: {
      allow: [
        '.',
        '../../canvas/Anima-canvas/pkg',
        '../../engine/Anima-engine/pkg'
      ]
    }
  }
})