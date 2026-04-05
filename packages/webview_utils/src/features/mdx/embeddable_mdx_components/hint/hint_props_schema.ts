import { z } from "zod";
import { Emphasis, emphasisForegroundTransform, emphasisSchema } from "../schemas/emphasis_schema";
import { childrenSchema } from "../schemas/children_schema";
import { getSizableObjectClasses, sizableObjectSchema } from "../schemas/sizable_object_schema";

export const embeddableHintComponentPropsSchema = emphasisSchema
    .merge(sizableObjectSchema)
    .extend({
        label: z.string({ message: "The 'label' property must be a string." }).default("Hint"),
        children: childrenSchema
    }).transform((c) => {
        return {
            ...c,
            containerClasses: getSizableObjectClasses(c),
            textGroup: emphasisForegroundTransform(Emphasis.Success)(c)
        }
    })



export type EmbeddableHintComponentPropsInput = z.input<typeof embeddableHintComponentPropsSchema>
