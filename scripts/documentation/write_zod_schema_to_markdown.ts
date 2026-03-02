import { zodSchemaSources } from "../../packages/webview_utils/src/features/in_content_documentation/methods/zod_to_xyz/zod_schema_sources";
import { writeZodSchemaMarkdown } from "../../packages/webview_utils/src/features/in_content_documentation/methods/zod_to_xyz/zod_to_markdown";

for (const item of zodSchemaSources) {
    writeZodSchemaMarkdown(item);
}
