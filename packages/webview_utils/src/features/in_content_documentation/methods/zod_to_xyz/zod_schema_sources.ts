import { ZodTypeAny } from "zod";
import { sizableObjectSchema } from "../../../mdx/embeddable_mdx_components/schemas/sizable_object_schema";
import { embeddableResponsivieGridPropsSchemaUnion } from "../../../mdx/embeddable_mdx_components/grid/embeddable_responsive_grid_props";


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
    },
    {
        schema: embeddableResponsivieGridPropsSchemaUnion,
        ignore: ["center"],
        outputPath: "embeddable-responsive-grid-properties.mdx"
    }
]
