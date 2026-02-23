import { z } from "zod";

export const reactNodeSchema = (fieldKey: string) => z.any().refine((a) => typeof a !== "undefined", {
    message: `The ${fieldKey} field is required. Try providing a string, number, or a Fragment.`
})
