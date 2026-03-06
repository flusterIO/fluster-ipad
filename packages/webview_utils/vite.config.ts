import { defineConfig } from "vite";
import dts from "vite-plugin-dts";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";
import path from "path";
import codspeedPlugin from "@codspeed/vitest-plugin";
import tailwind from "@tailwindcss/vite";

const isProd = process.env.FLUSTER_PROD_BUILD === "true";


// https://vite.dev/config/
export default defineConfig({
    plugins: [
        react(),
        tsconfigPaths(),
        tailwind(),
        dts({
            insertTypesEntry: true,
            copyDtsFiles: true,
        }),
        codspeedPlugin()
        // wasm(),
        // topLevelAwait()
    ],
    build: {
        lib: {
            name: "fluster",
            entry: {
                main: path.resolve(__dirname, "src/index.ts"),
                wasm: path.resolve(__dirname, "src/wasm_index.ts"),
            },
            formats: ["es"],
            fileName: (format, entryName) => `${entryName}.${format}.js`,
        },
        commonjsOptions: {
            transformMixedEsModules: true,
        },
        rollupOptions: {
            external: ["react", "react-dom", "react/jsx-runtime"],
            // output: {
            //     // Keep the assets (like .wasm) with predictable names
            //     assetFileNames: "assets/[name][extname]",
            // }
        },
        sourcemap: true,
        emptyOutDir: false,
        minify: isProd ? "esbuild" : undefined,
        cssMinify: isProd ? "lightningcss" : undefined
    },
    resolve: {
        alias: {
            path: "path-browserify",
            "@": path.resolve(__dirname, "./src/core"),
            "#": path.resolve(__dirname, "./src/features"),
        },
    },
});
