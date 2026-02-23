import { RecordWithTtl } from "dns";
import { z } from "zod";


export const emphasisOption = [
    "info",
    "error",
    "warn",
    "success",
    "primary",
] as const

export const emphasisSchema = (fieldKey: string) => z.record(z.enum(emphasisOption), z.boolean({
    message: `The ${fieldKey} field accepts a list of boolean properties. Some of these include: ${emphasisOption.map((o, i, a) => `"${o}"`).join(", ")}.`}
}).optional())


export type Emphasis = typeof emphasisOption[number]

export type EmphasisTransform = (k: Record<Emphasis, boolean>) => string


export const emphasisMapTransform = (classMap: Record<Emphasis, string>): EmphasisTransform => {
    return (data) => {
        const classes: string[] = []
        for (const k of emphasisOption) {
            if (data[k]) {
                classes.push(classMap[k])
            }
        }
        return classes.filter(a => a.length).join(" ")
    }
}
