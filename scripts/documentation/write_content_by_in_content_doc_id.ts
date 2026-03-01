import {
    InContentDocumentationId,
    InContentDocumentationFormat,
} from "../../packages/webview_utils/src/core/code_gen/typeshare/fluster_core_utilities";
import { markdownContentToDocumentationPair } from "./markdown_content_to_documentation_pair";
import path from "path";
import fs from "fs";

const root = path.resolve(__dirname, "../../");
const inputDir = path.resolve(root, "docs/in_content_docs");
const outputDir = path.resolve(
    root,
    "packages/rust/fluster_pre_parser/src/embedded/in_content_docs/",
);

export const inContentDocumentationIdToFileName = (
    id: InContentDocumentationId,
    format: InContentDocumentationFormat,
) => {
    return `${id}-${format}.mdx`;
};

export const writeContentToDocumentationId = (
    /**
     * The path relative to the monorepo root where the *input* documentation is contained.
     */
    inputFileName: string,
) => {
    const inputPath = path.resolve(inputDir, inputFileName);
    if (!fs.existsSync(inputPath)) {
        console.error(`Cannot continue... the ${inputFileName} file doesn't exist`);
        process.exit(1);
    }
    const fileContent = fs.readFileSync(inputPath, { encoding: "utf-8" });
    const group = markdownContentToDocumentationPair(fileContent);
    const shortOutputPath = path.resolve(
        outputDir,
        inputFileName.replace(".mdx", "-short.mdx"),
    );
    const longOutputPath = path.resolve(
        outputDir,
        inputFileName.replace(".mdx", "-full.mdx"),
    );

    fs.writeFileSync(shortOutputPath, group.short, { encoding: "utf-8" });
    console.log(`Wrote documentation from ${inputPath} to ${shortOutputPath}.`);
    fs.writeFileSync(longOutputPath, group.full, { encoding: "utf-8" });
    console.log(`Wrote documentation from ${inputPath} to ${longOutputPath}.`);
};

const getInputFiles = () => {
    const d = fs.readdirSync(inputDir, { recursive: false });
    return d;
};

const files = getInputFiles();

files.forEach((d) => {
    writeContentToDocumentationId(d);
});
