import { defineConfig } from "vite";
import dts from "vite-plugin-dts";
import tsconfigPaths from "vite-tsconfig-paths";
import path from "path";

const isProd = process.env.FLUSTER_PROD_BUILD === "true";

// https://vite.dev/config/
export default defineConfig({
    plugins: [
        tsconfigPaths(),
        dts({
            insertTypesEntry: true,
            copyDtsFiles: true,
        }),
        // wasm(),
        // topLevelAwait()
    ],
    build: {
        lib: {
            name: "fluster_internal_cli",
            entry: {
                main: path.resolve(__dirname, "src/main.ts"),
            },
            formats: ["es"],
            fileName: (format, entryName) => `${entryName}.${format}.js`,
        },
        commonjsOptions: {
            transformMixedEsModules: true,
        },
        sourcemap: true,
        emptyOutDir: false,
    },
});
