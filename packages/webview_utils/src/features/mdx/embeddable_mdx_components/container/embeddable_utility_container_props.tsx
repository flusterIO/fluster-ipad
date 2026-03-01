import { z } from "zod";
import { emphasisBackgroundTransform, emphasisSchema } from "../schemas/emphasis_schema";
import { getSizableObjectClasses, sizableObjectSchema } from "../schemas/sizable_object_schema";
import { embeddableResponsiveGridPropsSchema } from "../grid/embeddable_responsive_grid_props";

export const embeddableUtiltyContainerProps = sizableObjectSchema.merge(emphasisSchema).transform((c) => {
    return {
        emphasisBackgroundClasses: emphasisBackgroundTransform(undefined)(c),
        containerClasses: getSizableObjectClasses(c)
    }
})


export type EmbeddableUtilityContainerPropsInput = z.input<typeof embeddableResponsiveGridPropsSchema>
export type EmbeddableUtilityContainerPropsOutput = z.output<typeof embeddableResponsiveGridPropsSchema>
