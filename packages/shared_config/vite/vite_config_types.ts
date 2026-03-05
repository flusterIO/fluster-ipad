import { BuildEnvironmentOptions } from "vite";

export interface WebviewViteConfig {
    singleFile?: boolean;
    outputDir: string;
    plugins?: {
        /**
         * @deprecated -- All tailwind is being handled by the utils package alone.
         */
        tailwind?: boolean
    },
    base?: string;
    build?: Omit<BuildEnvironmentOptions, "outDir" | "minify" | "cssMinify">
// plugins: ReturnType<typeof getConfigPlugins>
}
