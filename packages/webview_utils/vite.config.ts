import { defineConfig } from "vite";
import dts from "vite-plugin-dts";
import react from "@vitejs/plugin-react";
import { resolve } from "path";
import tsconfigPaths from "vite-tsconfig-paths";
import path from "path";
import tailwind from "@tailwindcss/vite";

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
    ],
    build: {
        lib: {
            entry: resolve(__dirname, "src/index.ts"),
            // formats: ["es"],
            name: "fluster",
            fileName: (c) => `index.${c}.js`,
        },
        commonjsOptions: {
            transformMixedEsModules: true,
        },
        rollupOptions: {
            external: ["react", "react-dom"],
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
});
