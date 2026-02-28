import type { PluginOption, UserConfig } from "vite";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";
import { viteSingleFile } from "vite-plugin-singlefile";
import { loadMonorepoEnv } from "../build_utils/load_monorepo_env.ts";
import tailwind from "@tailwindcss/vite";
import htmlMinifier from 'vite-plugin-html-minifier'

/// Required to get the plugins to read the correct file paths.
// export const getConfigPlugins = () => {
//     return
// }

export interface WebviewViteConfig {
    singleFile?: boolean;
    outputDir: string;
    plugins?: {
        tailwind?: boolean
    },
    base?: string;
    // plugins: ReturnType<typeof getConfigPlugins>
}

export const getWebviewViteConfig = (config: WebviewViteConfig): UserConfig => {
    loadMonorepoEnv()
    const isProd = process.env.FLUSTER_PROD_BUILD === "true";
    let plugins: PluginOption[] = [react(), tsconfigPaths()];
    if (config.singleFile !== false) {
        plugins.push(viteSingleFile());
    }
    if (config.plugins?.tailwind) {
        plugins.push(tailwind)

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
        },
    } satisfies UserConfig;
};
