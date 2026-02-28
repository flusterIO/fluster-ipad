import { defineConfig } from "vite";
import {
  WebviewViteConfig,
  getWebviewViteConfig,
} from "@fluster/shared_config/webviewVite";

const config: WebviewViteConfig = {
  outputDir: "splitview_mdx_editor_ipad",
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
