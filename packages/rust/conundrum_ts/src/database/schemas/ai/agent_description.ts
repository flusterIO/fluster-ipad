import { z } from "zod";
import { nullableString } from "../schema_utils/string_schema_utils";

// export const mcpToolName = z.enum()

export const agentDescriptionSchema = z.object({
    id: z.string().uuid(),
    name: nullableString,
    model: z.string(),
    reasoning: z.boolean(),
    is_local: z.boolean(),
    instructions: nullableString,
    always_include_tools: z.string().array(),
    temperature_scalar: z.number().min(0).default(1),
    primary_task: z.string().nullable(),
});
