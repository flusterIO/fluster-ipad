import { embeddableComponentConfigs } from "../packages/webview_utils/src/features/mdx/embeddable_mdx_components/component_configs";
import fs from "fs";
import path from "path";

const monoRepoRoot = path.resolve(__dirname, "../");
const outputDir = path.resolve(
    monoRepoRoot,
    "./packages/rust/fluster_pre_parser/src/embedded/component_docs/",
);

const COMPONENT_DOCS_FULL_DOCS_SPLITTER = "<<FULLDOCS>>";

if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir);
}

interface DocGroup {
    full: string;
    short: string;
}

const toDocGroup = (noteContent: string): DocGroup => {
    const containsFullDocs = noteContent.includes(
        COMPONENT_DOCS_FULL_DOCS_SPLITTER,
    );
    if (containsFullDocs) {
        return {
            full: noteContent.replace(COMPONENT_DOCS_FULL_DOCS_SPLITTER, ""),
            short: noteContent.split(COMPONENT_DOCS_FULL_DOCS_SPLITTER)[0],
        };
    } else {
        return {
            full: noteContent.replace(COMPONENT_DOCS_FULL_DOCS_SPLITTER, ""),
            short: noteContent.replace(COMPONENT_DOCS_FULL_DOCS_SPLITTER, ""),
        };
    }
};

for (const config of embeddableComponentConfigs) {
    const dp = config.docsPath;
    if (dp) {
        const inputNotePath = path.resolve(monoRepoRoot, dp);
        const noteContent = fs.readFileSync(inputNotePath, { encoding: "utf-8" });
        const docGroup = toDocGroup(noteContent);
        const outputPathFull = path.resolve(outputDir, `${config.id}-full.mdx`);
        const outputPathShort = path.resolve(outputDir, `${config.id}-short.mdx`);
        fs.writeFileSync(outputPathShort, docGroup.short, { encoding: "utf-8" });
        fs.writeFileSync(outputPathFull, docGroup.full, { encoding: "utf-8" });
        console.info(
            `Wrote content for the ${config.title} component to the ${config.id}.mdx file.`,
        );
    } else {
        console.warn(
            `Found a component with the id "${config.title}" without component documentation`,
        );
    }
}
