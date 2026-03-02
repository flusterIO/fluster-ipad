import { z, ZodTypeAny } from "zod";
import fs from 'fs';
import path from 'path';
import { marked } from 'marked';
import { markedTerminal } from 'marked-terminal';
import assert from "node:assert";
import { ZodSchemaSource } from "./zod_schema_sources";

/* @ts-expect-error -- Just a useless development tool */
marked.use(markedTerminal({
}, { jsx: true }));

const logMarkdown = async (content: string): Promise<void> => {
    // const renderer = markedTerminal({}, { jsx: true })
    // const res = await parseInline(content)
    console.log(marked.parse(content))
}




const formatZodBodyTypeToMarkdown = (itemType: { typeName: string, values?: string[] }): string => {
    if (!itemType.typeName) {
        throw new Error(`Found a zod value without a vald type`)
    }

    console.log("itemType: ", itemType)
    if (itemType.typeName === "ZodBoolean") {
        return `
**Boolean** 
`
    }
    if (itemType.typeName === "ZodEnum") {
        let s = "\n**Enum:**\n\nOne of the following:\n\n"
        assert(Array.isArray(itemType.values), "Found an enum without values")
        for (const option of itemType.values!) {
            assert(["string", "number"].includes(typeof option), "Found a zod venum value that isn't a valid type.")
            console.log("option: ", option)
            s += `- ${option}\n`
        }
        return s
    }

    console.log("itemType reached here: ", itemType)
    throw new Error("Found a Zod object that was not handled properly.")
}



/**
 * For use in development only. This this is likely going to be super unreliable.
 */
export const zodSchemaToMarkdown = (
    schema: z.ZodTypeAny,
    /**
     * Required to ignore specifc keys like the error trigger property
     * used in the sizable model.
     */
    ignoreKeys: string[]
): string => {
    let s = ""
    const shape = schema._def.shape();
    const itemKeys = Object.keys(shape).filter((k) => !ignoreKeys.includes(k))
    for (const itemKey of itemKeys) {
        const item: ZodTypeAny = shape[itemKey]
        const description = item._def.description
        const itemType = item._def.schema._def.innerType._def
        if (!itemType) {
            console.log("itemKey: ", itemKey)
        }
        // console.log("description: ", description)

        console.log("Query: ", itemType)
        console.log("Object.keys(item): ", Object.keys(itemType))
        s += `
#### ${itemKey}
${formatZodBodyTypeToMarkdown(itemType)}

${description ?? ""}

`
    }

    logMarkdown(s)
    return s
}



export const writeZodSchemaMarkdown = (item: ZodSchemaSource) => {
    const output = zodSchemaToMarkdown(item.schema, item.ignore)
    const outputDir = path.resolve(__dirname, "../../../../../../../docs/generated/zod")
    fs.writeFileSync(path.join(outputDir, item.outputPath), output, { encoding: "utf-8" });

}
