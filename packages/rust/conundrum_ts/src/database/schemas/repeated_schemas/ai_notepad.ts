import { z } from "zod";

export const aiNotepadSchema = z.object({
    notes: z.string(),
    ai_generated_input: z.string(),
});

export const defaultAINotepadSchema: z.infer<typeof aiNotepadSchema> = {
    notes: "",
    ai_generated_input: "",
};
