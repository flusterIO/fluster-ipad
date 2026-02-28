import { defineConfig } from "vite";
import {
    WebviewViteConfig,
    getWebviewViteConfig,
} from "@fluster/shared_config/webviewVite";

const config: WebviewViteConfig = {
    outputDir: "splitview_mdx_editor",
    singleFile: false,
    plugins: {
        tailwind: false,
    },
    base: "./",
};

export default defineConfig(getWebviewViteConfig(config));
