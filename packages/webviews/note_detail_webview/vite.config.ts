import { defineConfig } from "vite";
import {
    WebviewViteConfig,
    getWebviewViteConfig,
} from "@fluster/shared_config/webviewVite";

const config: WebviewViteConfig = {
    outputDir: "note_detail_webview",
    // singleFile: false,
};

export default defineConfig(getWebviewViteConfig(config));
