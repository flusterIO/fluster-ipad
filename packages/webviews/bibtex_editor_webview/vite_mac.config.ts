import { defineConfig } from "vite";
import {
  WebviewViteConfig,
  getWebviewViteConfig,
} from "@fluster/shared_config/webviewVite";

const config: WebviewViteConfig = {
  outputDir: "bibtex_editor_webview_mac",
  singleFile: false,
  plugins: {
    tailwind: false,
  },
  base: "./",
  build: {
    rollupOptions: {
      input: {
        main: "./index_mac.html",
      },
    },
  },
};

export default defineConfig(getWebviewViteConfig(config));
