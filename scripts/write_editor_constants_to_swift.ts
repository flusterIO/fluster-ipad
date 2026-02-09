import { bundledThemesInfo } from "shiki";
import fs from "fs";
import path from "path";

const outputPath = path.resolve(
    __dirname,
    "../packages/swift/FlusterMdx/Sources/FlusterMdx/mdx_editor_constants.swift",
);

let darkThemes =
    "public enum EditorDarkTheme: String, CaseIterable, Hashable {\n";
let lightThemes =
    "public enum EditorLightTheme: String, CaseIterable, Hashable {\n";

export const capitalize = (s: string): string => {
    return `${s[0].toUpperCase()}${s.slice(1, s.length)}`;
};

const appendValue = (s: string, val: string): string => {
    return `${s}        case ${capitalize(
        val
            .replaceAll(" ", "_")
            .replaceAll(/[^a-zA-Z0-9]/gim, "")
            .toLowerCase(),
    )} = "${val}"\n`;
};

for (const item of bundledThemesInfo) {
    if (item.type === "dark") {
        darkThemes = appendValue(darkThemes, item.displayName);
    } else if (item.type === "light") {
        lightThemes = appendValue(lightThemes, item.displayName);
    }
}

const data = `
${darkThemes}}

${lightThemes}}
`;

fs.writeFileSync(outputPath, data, {
    encoding: "utf-8",
});

console.info("Wrote editor constants from typescript to swift successfully...");
