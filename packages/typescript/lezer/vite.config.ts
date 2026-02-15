import { defineConfig } from "vite";
import dts from "vite-plugin-dts";
import { resolve } from "path";
import tsconfigPaths from "vite-tsconfig-paths";
import path from "path";
import { lezer } from "@lezer/generator/rollup";

// https://vite.dev/config/
export default defineConfig({
    plugins: [
        tsconfigPaths(),
        lezer(),
        dts({
            insertTypesEntry: true,
            copyDtsFiles: true,
        }),
    ],
    build: {
        lib: {
            entry: resolve(__dirname, "src/index.ts"),
            // formats: ["es"],
            name: "lezer",
            fileName: (c) => `index.${c}.js`,
        },
        commonjsOptions: {
            transformMixedEsModules: true,
        },
        sourcemap: true,
        emptyOutDir: true,
    },
    resolve: {
        alias: {
            path: "path-browserify",
            "@": path.resolve(__dirname, "./src/core"),
            "#": path.resolve(__dirname, "./src/features"),
        },
    },
});
