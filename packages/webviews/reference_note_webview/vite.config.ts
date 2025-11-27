import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import tsconfigPaths from "vite-tsconfig-paths";
import { viteSingleFile } from "vite-plugin-singlefile";

export default defineConfig({
    plugins: [react(), tailwindcss(), tsconfigPaths(), viteSingleFile()],
    build: {
        outDir: "reference_note_webview",
    },
});
