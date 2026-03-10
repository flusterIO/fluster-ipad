import { defineConfig } from "vite";
import { WasmViteConfig, getWasmViteConfig } from "@fluster/shared_config/wasm";

const config: WasmViteConfig = {
    outputDir: "standalone_mdx_preview_ipad",
    build: {
        rollupOptions: {
            input: {
                main: "./index_ipad.html",
            },
        },
    },
};

export default defineConfig(getWasmViteConfig(config));
