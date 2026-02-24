import { z } from "zod";

export const classRecordSchema = (classes: readonly [string, ...string[]]) => {
    const o = z.object(Object.fromEntries(classes.map((k) => [k, z.string().optional()])), {
        message: `'classes' is a Record of <K, string> where K is any of ${classes.map((c) => `"${c}"`).join(", ")}`
    })
    return o
}
