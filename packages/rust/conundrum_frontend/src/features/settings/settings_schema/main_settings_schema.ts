import { z } from "zod";

export const mainSettingsSchema = z.object({
    name: z.object({
        first: z.string().optional(),
        middle: z.string().optional(),
        last: z.string().optional(),
    }),
});
