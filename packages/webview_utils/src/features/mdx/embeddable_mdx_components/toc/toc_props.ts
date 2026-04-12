import { z } from "zod";

export const tocProps = z.object({
    expanded: z.boolean().optional().describe("Automatically start to collapsable table of contents component in an expanded state."),
})
