import { defineConfig } from "vite";
import {
  WebviewViteConfig,
  getWebviewViteConfig,
} from "@fluster/shared_config/webviewVite";

const config: WebviewViteConfig = {
  outputDir: "bibtex_editor_webview_ipad",
  singleFile: false,
  plugins: {
    tailwind: false,
  },
  base: "./",
  build: {
    rollupOptions: {
      input: {
        main: "./index_ipad.html",
      },
    },
  },
};

export default defineConfig(getWebviewViteConfig(config));
