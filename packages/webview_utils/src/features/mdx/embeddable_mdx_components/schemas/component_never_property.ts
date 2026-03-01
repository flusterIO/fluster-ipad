import { z } from "zod"

export const componentNeverProperty = (msg: string) => {
    return z.any().refine((c) => {
        // Always throw error any time this property is used.
        return typeof c === "undefined"
    }, msg);
}
