import { defineConfig } from "vite";
import { WebviewViteConfig, getConfigPlugins, getWebviewViteConfig } from "@fluster/shared_config/webviewVite"

const config: WebviewViteConfig = {
    outputDir: "standalone_mdx_editor",
    // plugins: getConfigPlugins()
}

export default defineConfig(getWebviewViteConfig(config));
