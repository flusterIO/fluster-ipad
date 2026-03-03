import { ZodTypeAny } from "zod";
import { sizableObjectSchema } from "../../../mdx/embeddable_mdx_components/schemas/sizable_object_schema";
import { embeddableResponsivieGridPropsSchemaUnion } from "../../../mdx/embeddable_mdx_components/grid/embeddable_responsive_grid_props";
import { embeddableCardPropsSchema } from "../../../mdx/embeddable_mdx_components/card/embeddable_card_props";
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
    ...embeddableComponentConfigs.map((n) => {
        return {
            schema: n.schema,
            outputPath: `${n.name[0].toLowerCase()}_properties.mdx`
        } satisfies ZodSchemaSource
    })
]
