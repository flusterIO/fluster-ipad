import { z } from "zod";
import { withChildrenSchema } from "../schemas/children_schema";


export const hrPropsSchema = withChildrenSchema.or(
    z.object({
        content: z.string({ message: "The 'content' type is a string that can be placed in the center of your divider." }).optional(),
    })
)
