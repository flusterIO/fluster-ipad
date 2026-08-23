import { z } from "zod";
import { aiNotepadSchema } from "./repeated_schemas/ai_notepad";

export const userWorkspaceSchema: z.ZodObject<UserWorkspaceConfig> = z
    .object({
        root: z.string(),
        label: z.string().nullable(),
        respect_gitignore: z.boolean(),
        ignore_hidden: z.boolean(),
        resource_dir: z.string().optional().default("/resources"),
        ctime: z.string(),
    })
    .merge(aiNotepadSchema);
