import { embeddableComponentConfigs } from "../packages/webview_utils/src/features/mdx/embeddable_mdx_components/component_configs";
import fs from "fs";
import path from "path";
import { markdownContentToDocumentationPair } from "./documentation/markdown_content_to_documentation_pair";

const monoRepoRoot = path.resolve(__dirname, "../");
const outputDir = path.resolve(
    monoRepoRoot,
    "./packages/rust/fluster_pre_parser/src/embedded/component_docs/",
);

if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir);
}

for (const config of embeddableComponentConfigs) {
    const dp = config.docsPath;
    if (dp) {
        const inputNotePath = path.resolve(monoRepoRoot, dp);
        const noteContent = fs.readFileSync(inputNotePath, { encoding: "utf-8" });
        const docGroup = markdownContentToDocumentationPair(noteContent);
        const outputPathFull = path.resolve(outputDir, `${config.id}-full.mdx`);
        const outputPathShort = path.resolve(outputDir, `${config.id}-short.mdx`);
        fs.writeFileSync(outputPathShort, docGroup.short, { encoding: "utf-8" });
        fs.writeFileSync(outputPathFull, docGroup.full, { encoding: "utf-8" });
        console.info(
            `Wrote content for the ${config.name[0]} component to the ${config.id}.mdx file.`,
        );
    } else {
        console.warn(
            `Found a component with the id "${config.id}" without component documentation`,
        );
    }
}
