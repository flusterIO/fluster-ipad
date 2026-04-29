import { defineConfig } from "vite";
import dts from "vite-plugin-dts";
import tsconfigPaths from "vite-tsconfig-paths";
import path from "path";
import { viteSingleFile } from "vite-plugin-singlefile";

const isProd = process.env.FLUSTER_PROD_BUILD === "true";



// https://vite.dev/config/
export default defineConfig({
    plugins: [
        tsconfigPaths(),
        dts({
            insertTypesEntry: true,
            copyDtsFiles: true,
        }),
        viteSingleFile({
            useRecommendedBuildConfig: true
        })
        // wasm(),
        // topLevelAwait()
    ],
    build: {
        lib: {
            name: "fluster",
            entry: {
                main: path.resolve(__dirname, "src/output/html/glue/component_glue/conundrum_component_glue.ts"),
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
        outDir: path.resolve(__dirname, "../conundrum_web_assets/src/js"),
        sourcemap: true,
        emptyOutDir: false,
        minify: false,
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
