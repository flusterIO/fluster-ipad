import { z } from "zod";
import { emphasisSchema, getFirstEmphasisKey } from "../schemas/emphasis_schema";

export const ulPropsSchema = emphasisSchema.extend({
    thick: z.boolean({ message: "The 'thick' and 'thicker' fields are optional booleans." }).optional(),
    thicker: z.boolean({ message: "The 'thick' and 'thicker' fields are optional booleans." }).optional()
}).transform((c) => {
    const firstKey = getFirstEmphasisKey(c)
    if (!firstKey) {
        return [""]
    }
    const thicknessClass = c.thick ? "decoration-2" : c.thicker ? "decoration-4" : "decoration-1"
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
    }
})
