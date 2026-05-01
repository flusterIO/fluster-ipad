import fs from "fs";
import assert from "node:assert";
import path from "path";

interface Replacer {
    query: string;
    replaceWith: string;
    dontPanicIfExists?: string[];
}

const replacers: Record<
    string,
    {
        replacers?: Replacer[];
        header?: string;
    }
> = {
    "packages/webview_utils/src/core/code_gen/typeshare/conundrum.ts": {
        header: `export type ConundrumInt = number;
export type GridColumnsMap = Map<SizableOption, number>;`,
        replacers: [
            // {
            //     query: '| { tag: "Int", content: i128 }',
            //     dontPanicIfExists: ['| { tag: "Int", content: number }'],
            //     replaceWith: '| { tag: "Int", content: number }',
            // },
            {
                query: "DashMap<",
                dontPanicIfExists: ["Map<"],
                replaceWith: "Map<",
            },
        ],
    },
    "packages/rust/conundrum_swift/ConundrumSwift/Sources/ConundrumSwift/conundrum.swift":
    {
        replacers: [
            {
                query: "public enum SizableOption {",
                dontPanicIfExists: ["public enum SizableOption: Codable {"],
                replaceWith: "public enum SizableOption: Codable {",
            },
            {
                query: "public struct DocumentSpan {",
                dontPanicIfExists: ["public struct DocumentSpan: Codable {"],
                replaceWith: "public struct DocumentSpan: Codable {",
            },
            {
                query: "public struct ConundrumError {",
                dontPanicIfExists: ["public struct ConundrumError: Codable {"],
                replaceWith: "public struct ConundrumError: Codable {",
            },
            {
                query: "public enum ConundrumErrorVariant: Swift.Error {",
                dontPanicIfExists: [
                    "public enum ConundrumErrorVariant: Swift.Error, Codable {",
                ],
                replaceWith:
                    "public enum ConundrumErrorVariant: Swift.Error, Codable {",
            },
            {
                query: "public enum ConundrumErrorPurpose {",
                dontPanicIfExists: ["public enum ConundrumErrorPurpose: Codable {"],
                replaceWith: "public enum ConundrumErrorPurpose: Codable {",
            },
            {
                query: "public enum SupportedCodeBlockSyntax {",
                dontPanicIfExists: [
                    "public enum SupportedCodeBlockSyntax: Codable {",
                ],
                replaceWith: "public enum SupportedCodeBlockSyntax: Codable {",
            },
            {
                query: "public enum SupportedCodeBlockTheme {",
                dontPanicIfExists: [
                    "public enum SupportedCodeBlockTheme: String, Codable, CaseIterable {",
                ],
                replaceWith:
                    "public enum SupportedCodeBlockTheme: String, Codable, CaseIterable {",
            },
            {
                query: "public enum AnyComponentKey {",
                dontPanicIfExists: ["public enum AnyComponentKey: Codable {"],
                replaceWith: "public enum AnyComponentKey: Codable {",
            },
            {
                query: "public enum AutoInsertedComponentName {",
                dontPanicIfExists: [
                    "public enum AutoInsertedComponentName: Codable {",
                ],
                replaceWith: "public enum AutoInsertedComponentName: Codable {",
            },
            {
                query: "public enum EmbeddableComponentId {",
                dontPanicIfExists: ["public enum EmbeddableComponentId: Codable {"],
                replaceWith: "public enum EmbeddableComponentId: Codable {",
            },
            {
                query: "public enum WebGlueCodeGeneralFiles {",
                dontPanicIfExists: ["public enum WebGlueCodeGeneralFiles: Codable {"],
                replaceWith: "public enum WebGlueCodeGeneralFiles: Codable {",
            },
            {
                query: "public enum DocumentationComponentName {",
                dontPanicIfExists: [
                    "public enum DocumentationComponentName: Codable {",
                ],
                replaceWith: "public enum DocumentationComponentName: Codable {",
            },
            {
                query: "public struct RenderedFootnoteResult {",
                dontPanicIfExists: [
                    "public struct RenderedFootnoteResult: Codable {",
                ],
                replaceWith: "public struct RenderedFootnoteResult: Codable {",
            },
        ],
    },
    "packages/webview_utils/src/core/code_gen/typeshare/fluster_core_utilities.ts":
    {
        header:
            'import { type SizableOption, type ConundrumError } from "./conundrum";',
        replacers: [],
    },
    "packages/swift/FlusterData/Sources/FlusterData/code_gen/typeshare/FlusterCoreUtilities.swift":
    {
        header: "import ConundrumSwift",
        replacers: [
            {
                query: "public struct EditorChangeEvent {",
                dontPanicIfExists: ["public struct EditorChangeEvent: Codable {"],
                replaceWith: "public struct EditorChangeEvent: Codable {",
            },
            {
                query: "public enum WebviewFontSize: String, Codable {",
                dontPanicIfExists: [
                    "public enum WebviewFontSize: String, Codable, CaseIterable {",
                ],
                replaceWith:
                    "public enum WebviewFontSize: String, Codable, CaseIterable {",
            },
            // {
            //     query: "public enum CodeEditorTheme: String, Codable, CaseIterable {",
            //     dontPanicIfExists: ["public enum CodeEditorTheme: String, Codable, CaseIterable, Hashable {"],
            //     replaceWith: "public enum CodeEditorTheme: String, Codable, CaseIterable, Hashable {"
            // },
            {
                query: `public enum CodeEditorTheme: String, Codable {`,
                dontPanicIfExists: [
                    `public enum CodeEditorTheme: String, Codable, CaseIterable {`,
                ],
                replaceWith: `public enum CodeEditorTheme: String, Codable, CaseIterable {`,
            },
            {
                query: `public enum CodeEditorKeymap: String, Codable {`,

                dontPanicIfExists: [
                    `public enum CodeEditorKeymap: String, Codable, CaseIterable {`,
                ],
                replaceWith: `public enum CodeEditorKeymap: String, Codable, CaseIterable {`,
            },
            {
                query: `public enum FlusterTheme: String, Codable {`,

                dontPanicIfExists: [
                    `public enum FlusterTheme: String, Codable, CaseIterable {`,
                ],
                replaceWith: `public enum FlusterTheme: String, Codable, CaseIterable {`,
            },
            {
                query: "public enum EquationNumberingStrategy: String, Codable {",
                dontPanicIfExists: [
                    "public enum EquationNumberingStrategy: String, Codable, CaseIterable {",
                ],
                replaceWith:
                    "public enum EquationNumberingStrategy: String, Codable, CaseIterable {",
            },
        ],
    },
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

const writeHeader = (header: string, filePath: string): void => {
    const fp = path.resolve(path.resolve(__dirname, "../"), filePath);
    const content = fs.readFileSync(fp, { encoding: "utf-8" });
    const new_content = `${header}
${content}`;
    fs.writeFileSync(fp, new_content, { encoding: "utf-8" });
};

if (process.argv.length < 3) {
    for (const fp in replacers) {
        const item = replacers[fp];
        replaceStuff(item.replacers, fp);
        const header = item.header;
        if (header) {
            writeHeader(header, fp);
        }
    }
} else {
    const fp = process.argv[2];
    const item = replacers[fp];
    if (item) {
        replaceStuff(item.replacers, fp);
        const header = item.header;
        if (header) {
            writeHeader(header, fp);
        }
    } else {
        console.error("No replacable item found. Can't replace anything.");
    }
}
