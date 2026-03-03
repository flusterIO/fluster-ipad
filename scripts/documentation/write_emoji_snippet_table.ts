import { emojiSnippetItems } from "../../packages/webview_utils/src/features/editor/code_editor/data/snippets/emoji_snippets";
import path from "path";
import fs from "fs";
const root = path.resolve(__dirname, "../../");

let s = `
| Emoji | Key | Category |
| ----- | --- | -------- |
`;

for (const item of emojiSnippetItems) {
    s += `| :${item.value}: | ${item.value} | ${item.category} |\n`;
}

fs.writeFileSync(path.resolve(root, "docs/generated/emoji_table.mdx"), s, {
    encoding: "utf-8",
});
