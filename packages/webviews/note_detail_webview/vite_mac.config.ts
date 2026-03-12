import { defineConfig } from "vite";
import { getWasmViteConfig, WasmViteConfig } from "@fluster/shared_config/wasm";

const config: WasmViteConfig = {
    outputDir: "note_detail_webview_mac",
    build: {
        rollupOptions: {
            input: {
                main: "./index_mac.html",
            },
        },
    },
};

export default defineConfig(getWasmViteConfig(config));
