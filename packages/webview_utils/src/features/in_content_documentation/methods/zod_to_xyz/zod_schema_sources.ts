import { ZodTypeAny } from "zod";
import { sizableObjectSchema } from "../../../mdx/embeddable_mdx_components/schemas/sizable_object_schema";


export interface ZodSchemaSource {
    schema: ZodTypeAny
    /**
     * Relative to the docs/generated/zod output directory.
     */
    outputPath: string;
    ignore: string[]
}

export const zodSchemaSources: ZodSchemaSource[] = [
    {
        schema: sizableObjectSchema,
        ignore: ["center"],
        outputPath: "sizable-object-properties.mdx"
    }
]
