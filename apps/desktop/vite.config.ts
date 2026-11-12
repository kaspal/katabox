/// <reference types="vitest/config" />
import os from "node:os";
import path from "node:path";
import process from "node:process";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(() => ({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: { "@": path.resolve(import.meta.dirname, "src") },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },

  build: {
    // A desktop app loads its bundle from disk; one ~600 kB chunk is fine.
    chunkSizeWarningLimit: 1024,
  },

  test: {
    environment: "jsdom",
    globals: false,
    setupFiles: ["./src/test/setup.ts"],
    css: false,
    // Room for the slow lazy-route tests under load (async waits are capped at 10 s in
    // setup.ts), and half the cores, so the suite does not starve itself on a busy
    // machine.
    testTimeout: 20_000,
    hookTimeout: 20_000,
    maxWorkers: Math.max(1, Math.floor(os.availableParallelism() / 2)),
  },
}));
