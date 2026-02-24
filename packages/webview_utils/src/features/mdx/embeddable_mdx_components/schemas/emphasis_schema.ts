import { z, ZodBoolean, ZodOptional } from "zod";

const _emphasisSchema = {
    info: z.boolean({ message: `The info field is a boolean.` }).optional(),
    error: z.boolean({ message: `The error field is a boolean.` }).optional(),
    warn: z.boolean({ message: `The warn field is a boolean.` }).optional(),
    success: z.boolean({ message: `The success field is a boolean.` }).optional(),
    primary: z.boolean({ message: `The primary field is a boolean.` }).optional(),
    important: z.boolean({ message: `The important field is a boolean.` }).optional(),
    research: z.boolean({ message: `The research field is a boolean.` }).optional(),
} satisfies Record<string, ZodOptional<ZodBoolean>>

export const emphasisSchema = z.object(_emphasisSchema)


export const getEmphasisOptions = () => Object.keys(_emphasisSchema) as (keyof typeof _emphasisSchema)[]

export type EmphasisSchema = z.infer<typeof emphasisSchema>
export type Emphasis = keyof EmphasisSchema

export type EmphasisTransform = (k: { [K in Emphasis]?: boolean }) => string

export const getFirstEmphasisKey = (data: { [K in Emphasis]?: boolean | undefined }): Emphasis | undefined => {
    for (const k of getEmphasisOptions()) {
        if (data[k] === true) {
            return k
        }
    }
}


export const emphasisClassMapTransform = (classMap: Record<Emphasis, string>): EmphasisTransform => {
    return (data) => {
        const firstKey = getFirstEmphasisKey(data)
        return firstKey ? classMap[firstKey] : ""
    }
}

export const emphasisMapTransform = (classMap: Record<Emphasis, string>): EmphasisTransform => {
    return (data) => {
        const classes: string[] = []
        for (const k of getEmphasisOptions()) {
            if (data[k]) {
                classes.push(classMap[k])
            }
        }
        return classes.filter(a => a.length).join(" ")
    }
}



/**
 * The transformation to be used in something like a `Hl`
 * component, *not* in something lke a `ColorText` component.
 */
export const emphasisBackgroundTransform = (defaultKey: Emphasis): EmphasisTransform => {
    return (k): string => {
        const firstKey = getFirstEmphasisKey(k) ?? defaultKey
        if (!firstKey) {
            return ""
        }
        switch (firstKey) {
            case "info":
                return "bg-emphasisInfo text-emphasisInfoForeground"
            case "error":
                return "bg-emphasisError text-emphasisErrorForeground"
            case "warn":
                return "bg-emphasisWarn text-emphasisWarnForeground"
            case "success":
                return "bg-emphasisSuccess text-emphasisSuccessForeground"
            case "important":
                return "bg-emphasisImportant text-emphasisImportantForeground"
            case "research":
                return "bg-emphasisResearch text-emphasisResearchForeground"
            case "primary":
                return "bg-primary text-primary-foreground"

        }
    }
}

