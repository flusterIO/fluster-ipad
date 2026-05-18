import { defineConfig, defaultExclude } from "vitest/config";
import dts from "vite-plugin-dts";
import tsconfigPaths from "vite-tsconfig-paths";
import react from "@vitejs/plugin-react";
import path from "path";

const isProd = process.env.FLUSTER_PROD_BUILD === "true";

export default defineConfig({
    plugins: [
        react({
            babel: {
                plugins: ["babel-plugin-react-compiler"],
            },
        }),
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
            sourceMap: true,
        },
        sourcemap: true,
        emptyOutDir: true,
        minify: isProd,
        cssMinify: isProd ? "lightningcss" : undefined,
        lib: {
            entry: {
                main: path.resolve(__dirname, "./src/initialize_conundrum_web.ts"),
                methods: path.resolve(__dirname, "./src/methods.ts"),
                providersNext: path.resolve(__dirname, "./src/providers/next/index.ts"),
                uiBlog: path.resolve(__dirname, "./src/prebuilt_ui/blog/index.ts"),
            },
            fileName(format, entryName) {
                return `${entryName}.${format}.js`;
            },
            name: "@conundrum/ts",
        },
        rollupOptions: {
            external: ["react", "react-dom", "react/jsx-runtime"],
            output: {
                globals: {
                    react: "React",
                    "react-dom": "ReactDOM",
                    "react/jsx-runtime": "jsxRuntime",
                },
            },
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
