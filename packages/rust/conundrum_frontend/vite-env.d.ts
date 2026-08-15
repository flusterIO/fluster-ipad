/// <reference types="vite/client" />

interface ViteTypeOptions {
    // Disallows unknown keys.
    strictImportMetaEnv: unknown
}

interface ImportMetaEnv {
    readonly CDRM_SERVER_PORT?: string | number
}

// interface ImportMeta {
//     readonly env: ImportMetaEnv
// }
