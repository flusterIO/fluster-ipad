import { defineConfig } from "vite";
import {
  WebviewViteConfig,
  getWebviewViteConfig,
} from "@fluster/shared_config/webviewVite";

const config: WebviewViteConfig = {
  outputDir: "dictionary_webview_ipad",
  plugins: {
    tailwind: false,
  },
  singleFile: false,
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
