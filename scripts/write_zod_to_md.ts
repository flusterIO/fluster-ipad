import { zodSchemaToMarkdown } from "zod-to-markdown";
import fs from "fs";
import { z } from "zod";
import { convertSchemas, formatModelsAsMarkdown } from "zod2md";
import path from "path";
import { ComponentPropsFunctionGetter } from "../packages/webview_utils/src/features/mdx/embeddable_component_utils/zod_to_md/zod_to_md";
import { admonitionPropsSchemaGetter } from "../packages/webview_utils/src/features/mdx/embeddable_mdx_components/admonition/admonition_props_schema_getter";

interface SchemaToMarkdownItem {
    title: string;
    getter: ComponentPropsFunctionGetter;
    /** As a path of the docs/generated/zod directory */
    outputPath: string;
    additionalContent?: string;
}

const getters: SchemaToMarkdownItem[] = [];

const outputDir = path.resolve(__dirname, "../docs/generated/zod/");

for (const g of getters) {
    const schema = g.getter();
    const outputPath = path.join(outputDir, g.outputPath);

    // convert array of Zod schemas (path determines heading, unless overidden)
    const models = convertSchemas([{ schema, path: "user.ts" }]);
    // render markdown with top-level heading
    const markdown = formatModelsAsMarkdown(models, { title: g.title });

    // string contents
    console.log(markdown);
    fs.writeFileSync(outputPath, markdown, {
        encoding: "utf-8",
    });
}

// example Zod schema input
const User = z.object({
    id: z.number(),
    name: z.string(),
});
