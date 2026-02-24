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
            return ["decoration-emphasisInfo", thicknessClass]
        case "error":
            return ["decoration-emphasisError", thicknessClass]
        case "warn":
            return ["decoration-emphasisWarn", thicknessClass]
        case "success":
            return ["decoration-emphasisSuccess", thicknessClass]
        case "important":
            return ["decoration-emphasisImportant", thicknessClass]
        case "research":
            return ["decoration-emphasisResearch", thicknessClass]
        case "primary":
            return ["decoration-primary", thicknessClass]
        case "highlight":
            return ["decoration-emphasisHighlight", thicknessClass]
    }
})
