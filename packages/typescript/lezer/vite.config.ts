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
        rollupOptions: {
            external: ["react", "react-dom"],
        },
        sourcemap: true,
        emptyOutDir: true,
    },
    resolve: {
        alias: {
            path: "path-browserify",
            "@codemirror/state": path.resolve(
                __dirname,
                "../../../node_modules/@codemirror/state/dist/index.cjs",
            ),
            "@": path.resolve(__dirname, "./src/core"),
            "#": path.resolve(__dirname, "./src/features"),
        },
        dedupe: [
            "@lezer/common",
            "@lezer/lr",
            "@lezer/highlight",
            "@codemirror/language",
            "@codemirror/state",
            "@codemirror/view",
        ],
    },
});
