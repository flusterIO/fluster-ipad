import { defineConfig } from "vite";
import { WasmViteConfig, getWasmViteConfig } from "@fluster/shared_config/wasm";

const config: WasmViteConfig = {
  outputDir: "bibtex_editor_webview_ipad",
  build: {
    rollupOptions: {
      input: {
        main: "./index_ipad.html",
      },
    },
  },
};

export default defineConfig(getWasmViteConfig(config));
