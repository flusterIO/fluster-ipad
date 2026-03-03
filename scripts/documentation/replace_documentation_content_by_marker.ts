import assert from "node:assert";
import fs from "fs";
import path from "path";

const root = path.resolve(__dirname, "../../");

/**
 * Replaces content in the note using `<<DOCUMENTATION_CONSTANT:>>` syntax.
 */
export const replaceDocumentationContentMarkerByFilePath = (
    content: string,
): string => {
    const regex = /<<DOCUMENTATION_CONSTANT:(.*?)>>/g;
    let match;
    while ((match = regex.exec(content)) !== null) {
        let filePath = match[1];
        console.log("match: ", match.groups);
        console.log("filePath: ", filePath);
        const outputPath = path.resolve(root, filePath);
        assert(fs.existsSync(outputPath), `Failed to find file at ${outputPath}`);
        const generatedContentToInsert = fs.readFileSync(outputPath, {
            encoding: "utf-8",
        });
        content = content.replaceAll(match[0], generatedContentToInsert);
    }

    return content;
};
