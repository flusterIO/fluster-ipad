import { z } from "zod";
import { emphasisForegroundTransform, emphasisSchema } from "../../schemas/emphasis_schema";

export const equationReferencePropsSchema = emphasisSchema.extend({
    id: z.string({ message: "The `id` field is a string that can be referenced later to use this equation's number." }).min(3, "Please make this id at least 3 characters long for specificity's sake."),
    idx: z.number().int().describe("Automatically inserted by the compiler, this represents the index of the equation."),
    /// The user passes it in as `sub`, but the transpler transpiles it to `subscript`.
    subscript: z.boolean().optional(),
    /// The user passes it in as `super`, but the transpler transpiles it to `superscript` to get around that pesky keyword.
    superscript: z.boolean().optional()
}).transform((c) => {
    return {
        ...c,
        emphasisClasses: emphasisForegroundTransform(undefined)(c)
    }
})


export type EquationReferenceProps = z.input<typeof equationReferencePropsSchema>;
