import type { PluginOption, UserConfig } from 'vite';
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import htmlMinifier from 'vite-plugin-html-minifier'
import type { WebviewViteConfig } from "./vite_config_types"

export type WasmViteConfig = Pick<WebviewViteConfig, "outputDir" | "build"> & { plugins?: [] }

// export const getWasmViteConfig = (opts: WasmViteConfig): UserConfig => {
//     const isProd = process.env.FLUSTER_PROD_BUILD === "true";

//     let plugins: PluginOption[] = [
//         react(),
//         tsconfigPaths(),
//         wasm(),
//         topLevelAwait(),
//         ...(opts.plugins ?? [])
//     ];
//     if (isProd) {
//         plugins.push(htmlMinifier({
//             minify: true,
//         }))
//     }
//     return {
//         plugins,
//         base: "./",
//         build: {
//             ...opts.build,
//             minify: isProd ? "esbuild" : undefined,
//             cssMinify: isProd ? "lightningcss" : undefined,
//             outDir: opts.outputDir,
//             assetsDir: "./",
//             // Ensures the wasm file is treated as a static asset
//             assetsInlineLimit: 0
//         },
//         optimizeDeps: {
//             // Force Vite to pre-bundle the WASM package
//             exclude: ['@fluster/wasm']
//         }
//     }
// }

export const getWasmViteConfig = (opts: WasmViteConfig): UserConfig => {
    const isProd = process.env.FLUSTER_PROD_BUILD === "true";

    let plugins: PluginOption[] = [
        react(),
        tsconfigPaths(),
        // wasm(),
        // topLevelAwait(),
        ...(opts.plugins ?? [])
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
        // worker: {
        //     plugins: () => [wasm(), topLevelAwait()]
        // },
        // ADD THIS BLOCK:
        optimizeDeps: {
            // Stop Esbuild from trying to process packages that contain WASM/TLA
            exclude: [
                '@fluster/wasm', // Your raw WASM package
                '@fluster/utils' // The package name from your utils package.json
            ]
        }
    }
}
