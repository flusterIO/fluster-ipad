import { z } from "zod";


export const hrPropsSchema = z.object({
    content: z.string({ message: "The 'content' type is a string that can be placed in the center of your divider." }).optional(),
})
