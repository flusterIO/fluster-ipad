import type { BuildEnvironmentOptions, PluginOption, UserConfig } from "vite";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";
import { viteSingleFile } from "vite-plugin-singlefile";
import { loadMonorepoEnv } from "../build_utils/load_monorepo_env.ts";
import htmlMinifier from 'vite-plugin-html-minifier'
import type { WebviewViteConfig } from "./vite_config_types"

/// Required to get the plugins to read the correct file paths.
// export const getConfigPlugins = () => {
//     return
// }


export const getWebviewViteConfig = (config: WebviewViteConfig): UserConfig => {

    loadMonorepoEnv()
    const isProd = process.env.FLUSTER_PROD_BUILD === "true";
    let plugins: PluginOption[] = [react(), tsconfigPaths()];
    if (config.singleFile !== false) {
        plugins.push(viteSingleFile());
    }
    if (isProd) {
        plugins.push(htmlMinifier({
            minify: true,
        }))
    }
    return {
        base: config.base,
        plugins,
        build: {
            outDir: config.outputDir,
            minify: isProd ? "esbuild" : undefined,
            cssMinify: isProd ? "lightningcss" : undefined,
            ...config.build
        },
    } satisfies UserConfig;
};
