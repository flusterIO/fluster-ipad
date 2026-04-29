/// <reference types="vite/client" />

interface ViteTypeOptions {
    // Disallows unknown keys.
    strictImportMetaEnv: unknown
}

interface ImportMetaEnv {
    readonly CDRM_COMPONENT_NAME: string
}

// interface ImportMeta {
//     readonly env: ImportMetaEnv
// }
