import fs from "fs";
import path from "path";
import { embeddableComponentConfigs } from "../../packages/webview_utils/src/features/mdx/embeddable_mdx_components/component_configs";

let content = `
## Components

\`\`\`long-docs
 All components have provided snippets and documentation available via the \`??\` syntax.
\`\`\`

`;

for (const k of embeddableComponentConfigs) {
    content += `- ${k.name[0]}\n`;
    if (k.desc) {
        content += `  - ${k.desc!}\n`;
    }
}

console.log(content);

const outputPath = path.resolve(
    __dirname,
    "../../docs/in_content_docs/generated-component-list.mdx",
);

fs.writeFileSync(outputPath, content, { encoding: "utf-8" });
