import { defineConfig } from "vite";
import {
    getWasmViteConfig,
    WasmViteConfig,
} from "@fluster/shared_config/wasm";
import path from 'path'

const config: WasmViteConfig = {
    outputDir: "bibtex_editor_webview_mac",
    wasmPackagePath: path.resolve(__dirname, "../../rust/wasm/fluster_wasm"),
    build: {
        rollupOptions: {
            input: {
                main: "./index_mac.html",
            },
        },
    },
};

export default defineConfig(getWasmViteConfig(config));
