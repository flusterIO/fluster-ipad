import path from "path";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// https://vite.dev/config/
export default defineConfig({
    plugins: [react(), tailwindcss()],
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src/core/"),
            "#": path.resolve(__dirname, "./src/features/"),
            "@rspc/react": path.resolve(
                __dirname,
                "../../../node_modules/@rspc/react/dist/index.js",
            ),
            "@rspc/query-core": path.resolve(
                __dirname,
                "../../../node_modules/@rspc/query-core/dist/index.js",
            ),
        },
    },
});
