import { z } from "zod";
import { getSizableObjectClasses, sizableObjectSchema } from "../schemas/sizable_object_schema";

export const embeddableCardProps = sizableObjectSchema.extend({
    title: z.string({ message: "The 'title' property must be a string." }).min(1, "Please provide a title that isn't empty."),
    desc: z.string({ message: "The 'desc' property must be a string." }).optional().describe("An optional 'description' or 'subtitle' for the Card component."),
    shrink: z.boolean({ message: "'shrink' is a boolean property." }).optional().describe("'shrink' is a property that when set to true will apply a set of styles to make the card smaller and more compact."),
    centerBody: z.boolean({ message: "The 'centerBody' property is a boolean." }).optional().describe("'centerBody' center's the body of the card without affecting the card's title.")
}).transform((c) => {
    return {
        ...c,
        containerClasses: getSizableObjectClasses(c, ["centerContent"])
    }
})


export type EmbeddableCardProps = z.infer<typeof embeddableCardProps>
export type EmbeddableCardPropsInput = z.input<typeof embeddableCardProps>
