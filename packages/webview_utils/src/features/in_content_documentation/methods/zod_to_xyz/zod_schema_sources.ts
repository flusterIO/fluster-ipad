import { type ZodTypeAny } from "zod";
import { sizableObjectSchema } from "../../../mdx/embeddable_mdx_components/schemas/sizable_object_schema";
import { embeddableComponentConfigs } from "../../../mdx/embeddable_mdx_components/component_configs";


export interface ZodSchemaSource {
    schema: ZodTypeAny
    /**
     * Relative to the docs/generated/zod output directory.
     */
    outputPath: string;
}

export const zodSchemaSources: ZodSchemaSource[] = [
    {
        schema: sizableObjectSchema,
        outputPath: "sizable-object-properties.mdx"
    },
    ...embeddableComponentConfigs.filter(a => Boolean(a.schema)).map((n) => {
        return {
            /* eslint-disable-next-line  -- I just checked... */
            schema: n.schema!,
            outputPath: `${n.name[0].toLowerCase()}_properties.mdx`
        } satisfies ZodSchemaSource
    })
]
