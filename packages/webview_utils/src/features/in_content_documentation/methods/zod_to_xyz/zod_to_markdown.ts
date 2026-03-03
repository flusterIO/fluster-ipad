import fs from 'fs';
import path from 'path';
import { ZodSchemaSource } from "./zod_schema_sources";
import { ZodToMarkdownHandler } from './zod_to_markdown_handler';



const fileHeader = `
> The following section was generated directly from code. That means the properties will always reflect the current state of the application, but on the other hand, they might not be formatted as neatly as the rest of the content.

`


export const writeZodSchemaMarkdown = (item: ZodSchemaSource) => {
    const handler = new ZodToMarkdownHandler([], fileHeader)
    handler.zodSchemaToMarkdown(item.schema)
    const outputDir = path.resolve(__dirname, "../../../../../../../docs/generated/zod")
    fs.writeFileSync(path.join(outputDir, item.outputPath), handler.body, { encoding: "utf-8" });

}
