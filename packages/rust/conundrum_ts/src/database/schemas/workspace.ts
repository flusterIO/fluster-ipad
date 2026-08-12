import { z } from "zod";
import { aiNotepadSchema } from "./repeated_schemas/ai_notepad";

export const userWorkspaceSchema = z
  .object({
    root: z.string(),
    label: z.string().optional(),
    respect_gitignore: z.boolean(),
    ignore_hidden: z.boolean(),
    resource_dir: z.string().default("/resources"),
  })
  .merge(aiNotepadSchema);
