import { z } from "zod";

/**
 * Recursively parses a Zod schema into a Markdown table string.
 */
export function zodToMarkdown(
    schema: z.ZodTypeAny,
    title = "Schema Definition",
): string {
    let markdown = `## ${title}\n\n`;
    markdown += `| Field | Type | Required | Description |\n`;
    markdown += `| :--- | :--- | :--- | :--- |\n`;

    const parseObject = (obj: z.ZodObject<any>, prefix = "") => {
        const shape = obj.shape;

        for (const key in shape) {
            const field = shape[key];
            const name = prefix ? `${prefix}.${key}` : key;

            // Extract metadata
            const isOptional = field.isOptional();
            const description = field.description ?? "-";

            // Unwrap optional/nullable to get the core type
            let coreType = field;
            while (coreType._def.innerType) {
                coreType = coreType._def.innerType;
            }

            const typeName = coreType._def.typeName.replace("Zod", "");

            markdown += `| **${name}** | \`${typeName}\` | ${isOptional ? "No" : "Yes"} | ${description} |\n`;

            // Recurse if it's a nested object
            if (typeName === "Object") {
                parseObject(coreType as z.ZodObject<any>, name);
            }
        }
    };

    // Ensure we are starting with an object schema
    if (schema instanceof z.ZodObject) {
        parseObject(schema);
    } else {
        console.log("schema._def.: ", schema._def);
        markdown += `| - | \`${schema._def.type}\` | Yes | - |\n`;
    }

    return markdown;
}

export const zodSchemaToMarkdownString = (schema: z.ZodObject) => {
    // const fields = schema.typekk;
    const table = zodToMarkdown();
};
