import type { UserConfig } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import type { WebviewViteConfig } from "./vite_config_types"

export type WasmViteConfig = Pick<WebviewViteConfig, "outputDir" | "build">

export const getWasmViteConfig = (opts: WasmViteConfig): UserConfig => {
    const isProd = process.env.FLUSTER_PROD_BUILD === "true";
    return {
        plugins: [
            wasm(),
            topLevelAwait()
        ],
        base: "./",
        build: {
            ...opts.build,
            minify: isProd ? "esbuild" : undefined,
            cssMinify: isProd ? "lightningcss" : undefined,
            outDir: opts.outputDir,
            // Ensures the wasm file is treated as a static asset
            assetsInlineLimit: 0
        },
        optimizeDeps: {
            // Force Vite to pre-bundle the WASM package
            exclude: ['@fluster/wasm']
        }
    }
}
