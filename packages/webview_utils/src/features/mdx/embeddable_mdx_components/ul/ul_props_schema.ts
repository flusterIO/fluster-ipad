import { z } from "zod";
import { emphasisSchema, getFirstEmphasisKey } from "../schemas/emphasis_schema";

export const ulPropsSchema = emphasisSchema.extend({
    thick: z.boolean({ message: "The 'thick' and 'thin' fields are optional booleans." }).optional(),
    thin: z.boolean({ message: "The 'thick' and 'thin' fields are optional booleans." }).optional()
}).transform((c) => {
    const firstKey = getFirstEmphasisKey(c)
    if (!firstKey) {
        return [""]
    }
    const thicknessClass = c.thick ? "decoration-4" : c.thin ? "decoration-1" : "decoration-2"
    switch (firstKey) {
        case "info":
            return ["decoration-emphasis-info", thicknessClass]
        case "error":
            return ["decoration-emphasis-error", thicknessClass]
        case "warn":
            return ["decoration-emphasis-warn", thicknessClass]
        case "success":
            return ["decoration-emphasis-success", thicknessClass]
        case "important":
            return ["decoration-emphasis-important", thicknessClass]
        case "research":
            return ["decoration-emphasis-research", thicknessClass]
        case "primary":
            return ["decoration-primary", thicknessClass]
        case "highlight":
            return ["decoration-emphasis-highlight", thicknessClass]
        case "card":
            return ["decoration-fd-card", thicknessClass]
    }
})

export type ULProps = z.input<typeof ulPropsSchema>
