import type { PluginOption, UserConfig } from "vite";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";
import { viteSingleFile } from "vite-plugin-singlefile";

/// Required to get the plugins to read the correct file paths.
// export const getConfigPlugins = () => {
//     return
// }

export interface WebviewViteConfig {
    singleFile?: boolean;
    outputDir: string;
    // plugins: ReturnType<typeof getConfigPlugins>
}

export const getWebviewViteConfig = (config: WebviewViteConfig): UserConfig => {
    let plugins: PluginOption[] = [react(), tsconfigPaths()];
    if (config.singleFile !== false) {
        plugins.push(viteSingleFile());
    }
    return {
        plugins,
        build: {
            outDir: config.outputDir,
        },
    } satisfies UserConfig;
};
