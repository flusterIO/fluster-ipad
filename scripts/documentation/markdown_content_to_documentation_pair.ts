import { DocumentationPair } from "./types";

const codeBlockRegex = /(?:^|\n)(`{3,})(.*?)\n([\s\S]*?)\n\1/g;

const getMatches = (input: string) => {
    const matches = [...input.matchAll(codeBlockRegex)];

    return matches.map((match) => ({
        fence: match[1],
        label: match[2].trim(),
        content: match[3],
        fullMatch: match[0].trim(),
    }));
};

/**
 * Takes the file content of some internal documentation, and removes the long-doc code blocks accordingly.
 */
export const markdownContentToDocumentationPair = (
    content: string,
): DocumentationPair => {
    const matches = getMatches(content);
    let shortDocs = content;
    matches.forEach((block) => {
        if (block.label === "docs-long") {
            shortDocs = shortDocs.replace(block.fullMatch, "");
        }
    });
    let longDocs = content;
    matches.forEach((block) => {
        if (block.label === "docs-long") {
            longDocs = longDocs.replace(block.fullMatch, block.content);
        }
    });
    return {
        short: shortDocs,
        full: longDocs,
    };
};
