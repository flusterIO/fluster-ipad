import fs from "fs";
import assert from "node:assert";
import path from "path";

interface Replacer {
    query: string;
    replaceWith: string;
    dontPanicIfExists?: string[];
}

const replacers: Record<string, Replacer[]> = {
    "packages/swift/FlusterData/Sources/FlusterData/code_gen/typeshare/FlusterCoreUtilities.swift":
        [
            {
                query: "public struct EditorChangeEvent {",
                dontPanicIfExists: ["public struct EditorChangeEvent: Codable {"],
                replaceWith: "public struct EditorChangeEvent: Codable {",
            },
            {
                query: "public enum WebviewFontSize: String, Codable {",
                dontPanicIfExists: ["public enum WebviewFontSize: String, Codable, CaseIterable {"],
                replaceWith: "public enum WebviewFontSize: String, Codable, CaseIterable {",
            },
            {
                query: `public enum CodeEditorTheme: String, Codable {`,
                replaceWith: `public enum CodeEditorTheme: String, Codable, CaseIterable {`,
            },
            {
                query: `public enum CodeEditorKeymap: String, Codable {`,
                replaceWith: `public enum CodeEditorKeymap: String, Codable, CaseIterable {`,
            },
            {
                query: `public enum FlusterTheme: String, Codable {`,
                replaceWith: `public enum FlusterTheme: String, Codable, CaseIterable {`,
            },
        ],
};

export const replaceStuff = (replacers: Replacer[], filePath: string) => {
    const fp = path.resolve(path.resolve(__dirname, "../"), filePath);
    let content = fs.readFileSync(fp, {
        encoding: "utf-8",
    });

    console.log("content: ", content);
    for (const replacer of replacers) {
        const queryExists = content.includes(replacer.query);
        if (
            !queryExists &&
            !replacer.dontPanicIfExists?.some((item) => content.includes(item))
        ) {
            assert(
                queryExists,
                `Attempted to modify generated content that doesn't exist: ${replacer.query}`,
            );
        }
        content = content.replaceAll(replacer.query, replacer.replaceWith);
    }

    fs.writeFileSync(fp, content, { encoding: "utf-8" });
};

for (const fp in replacers) {
    replaceStuff(replacers[fp], fp);
}
