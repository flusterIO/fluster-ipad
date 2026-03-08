/// <reference types="vite/client" />
import { type FlusterBuildEnvironment } from "./src/development/fluster_build_env"

interface ViteTypeOptions {
    // Disallows unknown keys.
    strictImportMetaEnv: unknown
}

interface ImportMetaEnv {
    readonly FLUSTER_BUILD_ENV: FlusterBuildEnvironment
}

// interface ImportMeta {
//     readonly env: ImportMetaEnv
// }
