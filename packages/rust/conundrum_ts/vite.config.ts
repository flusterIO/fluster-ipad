import { defineConfig, defaultExclude } from "vitest/config";
import dts from "vite-plugin-dts";
import tsconfigPaths from "vite-tsconfig-paths";
import path from "path";

export default defineConfig({
    plugins: [
        tsconfigPaths(),
        dts({
            insertTypesEntry: true,
            copyDtsFiles: true,
        }),
    ],
    build: {
        outDir: "dist",
        commonjsOptions: {
            transformMixedEsModules: true,
        },
        sourcemap: true,
        emptyOutDir: false,
        lib: {
            entry: {
                main: path.resolve(__dirname, "./src/initialize_conundrum_web.ts"),
                methods: path.resolve(__dirname, "./src/component_glue/methods.ts"),
            },
            fileName(format, entryName) {
                return `${entryName}.${format}.js`;
            },
            name: "@conundrum/ts",
        },
    },
    resolve: {
        alias: {
            path: "path-browserify",
            "@": path.resolve(__dirname, "./src/core"),
            "#": path.resolve(__dirname, "./src/features"),
        },
    },
    test: {
        globals: true,
        environment: "jsdom",
        setupFiles: ["./vitest.setup.ts"],
        exclude: [...defaultExclude],
        coverage: {
            provider: "v8",
            reporter: ["text", "json", "html"],
        },
    },
});
