import { sizableObjectSchema } from "../schemas/sizable_object_schema";
import { z } from "zod"

export const imageComponentPropsSchema = sizableObjectSchema.extend({
    src: z.string(),
    alt: z.string().optional(),
    caption: z.string().optional(),
    captionRight: z.boolean().optional(),
    captionLeft: z.boolean().optional(),
})
