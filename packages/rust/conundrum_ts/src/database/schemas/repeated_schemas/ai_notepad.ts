import { z } from "zod";

export const aiNotepadSchema = z.object({
    ai: z.object({
        notes: z.string(),
        ai_generated_input: z.string(),
    }),
});

export const defaultAINotepadSchema: z.infer<typeof aiNotepadSchema> = {
    ai: {
        notes: "",
        ai_generated_input: "",
    },
};
