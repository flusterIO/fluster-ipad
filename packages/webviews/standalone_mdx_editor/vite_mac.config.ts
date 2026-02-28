import { defineConfig } from "vite";
import {
  WebviewViteConfig,
  getWebviewViteConfig,
} from "@fluster/shared_config/webviewVite";

const config: WebviewViteConfig = {
  outputDir: "standalone_mdx_editor_mac",
  singleFile: false,
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
