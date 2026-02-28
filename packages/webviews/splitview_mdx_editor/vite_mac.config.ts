import { defineConfig } from "vite";
import {
  WebviewViteConfig,
  getWebviewViteConfig,
} from "@fluster/shared_config/webviewVite";

const config: WebviewViteConfig = {
  outputDir: "splitview_mdx_editor_mac",
  plugins: {
    tailwind: false,
  },
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
