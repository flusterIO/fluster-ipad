/// <reference types="vite/client" />
import { type FlusterBuildEnvironment } from "./src/development/fluster_build_env"

interface ViteTypeOptions {
    // Disallows unknown keys.
    strictImportMetaEnv: unknown
}

interface ImportMetaEnv {
    readonly FLUSTER_BUILD_ENV: FlusterBuildEnvironment
    readonly FLUSTER_PROD_BUILD: "true" | undefined
    readonly MATHJAX_FONT_URL: string;
}

// interface ImportMeta {
//     readonly env: ImportMetaEnv
// }
