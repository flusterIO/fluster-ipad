import { sizableObjectSchema } from "../schemas/sizable_object_schema";
import { z } from "zod"

export const quotePropsSchema = sizableObjectSchema.extend({
    source: z.string().optional(),
    sourceId: z.string().optional()
})
