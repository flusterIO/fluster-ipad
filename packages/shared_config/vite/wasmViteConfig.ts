import type { PluginOption, UserConfig } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";
import htmlMinifier from 'vite-plugin-html-minifier'
import type { WebviewViteConfig } from "./vite_config_types"
import path from 'path';
import wasmPack from 'vite-plugin-wasm-pack';
import assert from 'node:assert';

export type WasmViteConfig = Pick<WebviewViteConfig, "outputDir" | "build"> & { plugins?: [] } & {
    /**
     * The *absolute* path to the wasm package.
     */
    wasmPackagePath: string
}

export const getWasmViteConfig = (opts: WasmViteConfig): UserConfig => {
    const isProd = process.env.FLUSTER_PROD_BUILD === "true";

    assert(path.isAbsolute(opts.wasmPackagePath), "The wasmPackagePath must be absolute.")

    let plugins: PluginOption[] = [
        react(),
        tsconfigPaths(),
        ...(opts.plugins ?? []),
        // wasm(),
        // topLevelAwait(),
        // wasmPack(opts.wasmPackagePath)
    ];

    if (isProd) {
        plugins.push(htmlMinifier({ minify: true }));
    }

    return {
        plugins,
        base: "./",
        build: {
            ...opts.build,
            target: 'esnext',
            minify: isProd ? "esbuild" : false,
            outDir: opts.outputDir,
            assetsDir: "./",
            assetsInlineLimit: 0,
            rollupOptions: {
                ...opts.build?.rollupOptions,
                output: {
                    entryFileNames: `[name].js`,
                    chunkFileNames: `[name].js`,
                    assetFileNames: `[name].[ext]`
                }
            }
        },
        optimizeDeps: {
            exclude: [
                '@fluster/wasm',
                '@fluster/webview_utils'
            ]
        }
    }
}
