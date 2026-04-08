import { z } from "zod";
import { emphasisForegroundTransform, emphasisSchema } from "../../schemas/emphasis_schema";

export const equationReferencePropsSchema = emphasisSchema.extend({
    id: z.string({ message: "The `id` field is a string that can be referenced later to use this equation's number." }).min(3, "Please make this id at least 3 characters long for specificity's sake."),
    idx: z.number().int().describe("Automatically inserted by the compiler, this represents the index of the equation."),
    // anchor: z.boolean().optional().describe("Set this to true when the component is actually wrapping the equation, leave it as false when the component is meant to _apply_ the equation's number... true when it's meant to _read_ it. "),
    subscript: z.boolean().optional(),
    superscript: z.boolean().optional()
}).transform((c) => {
    return {
        ...c,
        emphasisClasses: emphasisForegroundTransform(undefined)(c)
    }
})


export type EquationReferenceProps = z.input<typeof equationReferencePropsSchema>;
