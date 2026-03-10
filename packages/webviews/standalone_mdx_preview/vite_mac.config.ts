import { defineConfig } from "vite";
import { getWasmViteConfig, WasmViteConfig } from "@fluster/shared_config/wasm";

const config: WasmViteConfig = {
  outputDir: "standalone_mdx_preview_mac",
  wasmPackagePath: undefined,
  build: {
    rollupOptions: {
      input: {
        main: "./index_mac.html",
      },
    },
  },
};

export default defineConfig(getWasmViteConfig(config));
