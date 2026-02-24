import { z } from "zod";
import { emphasisSchema, getFirstEmphasisKey } from "../schemas/emphasis_schema";

export const ulPropsSchema = emphasisSchema.transform((c) => {
    const firstKey = getFirstEmphasisKey(c)
    if (!firstKey) {
        return ""
    }
    switch (firstKey) {
        case "info":
            return "decoration-emphasisInfo"
        case "error":
            return "decoration-emphasisError"
        case "warn":
            return "decoration-emphasisWarn"
        case "success":
            return "decoration-emphasisSuccess"
        case "important":
            return "decoration-emphasisImportant"
        case "research":
            return "decoration-emphasisResearch"
        case "primary":
            return "decoration-primary"
    }
})
