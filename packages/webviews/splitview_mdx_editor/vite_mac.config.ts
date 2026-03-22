import { defineConfig } from "vite";
import {
  type WasmViteConfig,
  getWasmViteConfig,
} from "@fluster/shared_config/wasm";
import path from "path";

const config: WasmViteConfig = {
  outputDir: "splitview_mdx_editor_mac",
  wasmPackagePath: path.resolve(__dirname, "../../rust/wasm/fluster_wasm"),
  build: {
    rollupOptions: {
      input: {
        main: "./index_mac.html",
      },
    },
  },
};

export default defineConfig(getWasmViteConfig(config));
