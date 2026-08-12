import { z } from "zod";

export const generalPromptSchema = z.object({
    prompt: z.string().default(""),
});
