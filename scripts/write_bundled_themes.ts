import path from "path";
import fs from "fs";
import { bundledThemesInfo } from "shiki";

const outputPath = path.join(
    __dirname,
    "..",
    "packages",
    "webview_utils",
    "src",
    "desktop",
    "features",
    "code",
    "data",
    "bundled_themes.ts",
);

let lightThemes = "export const lightSyntaxThemes = [\n";
let darkThemes = "export const darkSyntaxThemes = [\n";

for (const item of bundledThemesInfo) {
    if (item.type === "dark") {
        darkThemes += `    "${item.displayName}",\n`;
    } else {
        lightThemes += `    "${item.displayName}",\n`;
    }
}

darkThemes += "] as const;";
lightThemes += "] as const;";

fs.writeFileSync(
    outputPath,
    `
${lightThemes}

${darkThemes}

export type BundledFlusterTheme = (typeof lightSyntaxThemes[number]) | (typeof darkSyntaxThemes[number]);
`,
    {
        encoding: "utf8",
    },
);
