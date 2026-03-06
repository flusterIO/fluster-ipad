import { defineConfig, defaultExclude } from "vitest/config";
import dts from "vite-plugin-dts";
import react from "@vitejs/plugin-react";
import tsconfigPaths from "vite-tsconfig-paths";
import path from "path";
import tailwind from "@tailwindcss/vite";

export default defineConfig({
    plugins: [
        react(),
        tsconfigPaths(),
        tailwind(),
        dts({
            insertTypesEntry: true,
            copyDtsFiles: true,
        }),
    ],
    build: {
        outDir: "dist_dev",
        commonjsOptions: {
            transformMixedEsModules: true,
        },
        sourcemap: true,
        emptyOutDir: false,
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
        setupFiles: ['./vitest.setup.ts'],
        exclude: [...defaultExclude],
        coverage: {
            provider: "v8",
            reporter: ["text", "json", "html"]
        }
    }
});
