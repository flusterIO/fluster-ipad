import { bundledThemesInfo } from "shiki";
import fs from "fs";
import path from "path";

const outputPath = path.resolve(
    __dirname,
    "../packages/webview_utils/src/features/split_view_editor/data/editor_theme_values.ts",
);

let editorLightThemesType = "export type EditorLightThemeValue = ";
let editorDarkThemesType = "export type EditorDarkThemeValue = ";

let editorLightThemesArray =
    "export const getEditorLightThemeValues = () => {\n     return [\n";
let editorDarkThemesArray =
    "export const getEditorDarkThemeValues = () => {\n     return [\n";

const appendType = (body: string, val: string): string => {
    return `${body} | "${val}"`;
};

const appendArray = (body: string, val: string): string => {
    return `${body}        "${val}",\n`;
};

for (const item of bundledThemesInfo) {
    if (item.type === "dark") {
        editorDarkThemesType = appendType(editorDarkThemesType, item.displayName);
        editorDarkThemesArray = appendArray(
            editorDarkThemesArray,
            item.displayName,
        );
    } else if (item.type === "light") {
        editorLightThemesType = appendType(editorLightThemesType, item.displayName);
        editorLightThemesArray = appendArray(
            editorLightThemesArray,
            item.displayName,
        );
    }
}

const data = `// This file was auto-generated during the build process. Any changes made here will be lost.

${editorDarkThemesType}

${editorLightThemesType}

${editorLightThemesArray}    ] as EditorLightThemeValue[];
}


${editorDarkThemesArray}    ] as EditorDarkThemeValue[];
}

export const getAllEditorThemes = () => {
    return [...getEditorDarkThemeValues(), ...getEditorLightThemeValues()]
}

export type BundledThemeValue = EditorLightThemeValue | EditorDarkThemeValue

`;

fs.writeFileSync(outputPath, data, {
    encoding: "utf-8",
});

console.info("Wrote editor constants to typescript successfully...");
