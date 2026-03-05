import { defineConfig } from "vite";
import {
    type WasmViteConfig,
    getWasmViteConfig,
} from "@fluster/shared_config/vite";

const config: WasmViteConfig = {
    outputDir: "splitview_mdx_editor_mac",
    build: {
        rollupOptions: {
            input: {
                main: "./index_mac.html",
            },
        },
    },
};

export default defineConfig(getWasmViteConfig(config));
