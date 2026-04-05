import { z } from "zod";
import { Emphasis, emphasisSchema, getFirstEmphasisKey } from "../schemas/emphasis_schema";

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
        case Emphasis.Info:
            return ["decoration-emphasis-info", thicknessClass]
        case Emphasis.Error:
            return ["decoration-emphasis-error", thicknessClass]
        case Emphasis.Warn:
            return ["decoration-emphasis-warn", thicknessClass]
        case Emphasis.Success:
            return ["decoration-emphasis-success", thicknessClass]
        case Emphasis.Important:
            return ["decoration-emphasis-important", thicknessClass]
        case Emphasis.Research:
            return ["decoration-emphasis-research", thicknessClass]
        case Emphasis.Primary:
            return ["decoration-primary", thicknessClass]
        case Emphasis.Highlight:
            return ["decoration-emphasis-highlight", thicknessClass]
        case Emphasis.Card:
            return ["decoration-fd-card", thicknessClass]
    }
})

export type ULProps = z.input<typeof ulPropsSchema>
