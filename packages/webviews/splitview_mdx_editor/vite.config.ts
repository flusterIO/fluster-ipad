import { defineConfig } from "vite";
import {
    WebviewViteConfig,
    getWebviewViteConfig,
} from "@fluster/shared_config/webviewVite";

const config: WebviewViteConfig = {
    outputDir: "splitview_mdx_editor",
    // plugins: getConfigPlugins()
};

export default defineConfig(getWebviewViteConfig(config));
