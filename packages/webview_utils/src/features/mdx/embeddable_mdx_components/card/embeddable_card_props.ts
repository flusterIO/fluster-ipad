import { z } from "zod";
import { getSizableObjectClasses, sizableObjectSchema } from "../schemas/sizable_object_schema";

export const embeddableCardProps = sizableObjectSchema.extend({
    title: z.string({ message: "The 'title' property must be a string." }),
    desc: z.string({ message: "The 'desc' property must be a string." }).optional(),
}).transform((c) => {
    return {
        ...c,
        containerClasses: getSizableObjectClasses(c)
    }
})


export type EmbeddableCardProps = z.infer<typeof embeddableCardProps>
export type EmbeddableCardPropsInput = z.input<typeof embeddableCardProps>
