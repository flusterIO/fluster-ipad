import { getBaseTailwindConfig } from "@fluster/shared_config/getTailwindBaseConfig";
import path from "path";

export default getBaseTailwindConfig({
    content: [
        path.resolve(__dirname, "./src/**/*.{ts,tsx,mdx}"),
        // "./src/*.{ts,tsx,mdx}",
        // "../../webview_utils/src/**/*.{ts,tsx,mdx}",
        // "packages/webview_utils/**/*.{ts,tsx,mdx}",
    ],
    includeWebUtils: true,
});
