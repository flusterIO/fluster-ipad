import type { UserConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import tsconfigPaths from "vite-tsconfig-paths";
import { viteSingleFile } from "vite-plugin-singlefile";


/// Required to get the plugins to read the correct file paths.
// export const getConfigPlugins = () => {
//     return 
// }

export interface WebviewViteConfig {
    outputDir: string
    // plugins: ReturnType<typeof getConfigPlugins>
}


export const getWebviewViteConfig = (config: WebviewViteConfig): UserConfig => {
    return {
        plugins: [react(), tailwindcss(), tsconfigPaths(), viteSingleFile()],
        build: {
            outDir: config.outputDir,
        }
    } satisfies UserConfig
}
