import type { PluginOption, UserConfig } from 'vite';
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";
import htmlMinifier from 'vite-plugin-html-minifier'
import type { WebviewViteConfig } from "./vite_config_types"

export type WasmViteConfig = Pick<WebviewViteConfig, "outputDir" | "build"> & { plugins?: [] } & {
    /**
     * The *absolute* path to the wasm package.
     * @deprecated -- Don't use this.
     */
    wasmPackagePath?: string
}

export const getWasmViteConfig = (opts: WasmViteConfig): UserConfig => {
    const isProd = process.env.FLUSTER_PROD_BUILD === "true";

    let plugins: PluginOption[] = [
        react(),
        tsconfigPaths(),
        ...(opts.plugins ?? []),
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
