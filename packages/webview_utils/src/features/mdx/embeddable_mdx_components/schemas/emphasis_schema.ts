import { z, ZodBoolean, ZodOptional } from "zod";

const _emphasisSchema = {
    info: z.boolean({ message: `The info field is a boolean.` }).optional(),
    error: z.boolean({ message: `The error field is a boolean.` }).optional(),
    warn: z.boolean({ message: `The warn field is a boolean.` }).optional(),
    success: z.boolean({ message: `The success field is a boolean.` }).optional(),
    primary: z.boolean({ message: `The primary field is a boolean.` }).optional(),
    important: z.boolean({ message: `The important field is a boolean.` }).optional(),
    research: z.boolean({ message: `The research field is a boolean.` }).optional(),
    highlight: z.boolean({ message: `The highlight field is a boolean.` }).optional(),
    card: z.boolean({ message: `The 'card' field is a boolean.` }).optional(),
} satisfies Record<string, ZodOptional<ZodBoolean>>

export const emphasisSchema = z.object(_emphasisSchema, {
    message: `All components that accept an 'emphasis' take a list of optional boolean properties, where this list is: ${Object.keys(_emphasisSchema).map((k) => `"${k}"`).join(", ")}`
})


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


export const emphasisToBackgroundClasses = (emphasis: Emphasis) => {
    switch (emphasis) {
        case "info":
            return "bg-emphasis-info text-emphasis-info-foreground [&>p]:text-emphasis-info-foreground"
        case "error":
            return "bg-emphasis-error text-emphasis-error-foreground [&>p]:text-emphasis-error-foreground"
        case "warn":
            return "bg-emphasis-warn text-emphasis-warn-foreground [&>p]:text-emphasis-warn-foreground"
        case "success":
            return "bg-emphasis-success text-emphasis-success-foreground [&>p]:text-emphasis-success-foreground"
        case "important":
            return "bg-emphasis-important text-emphasis-important-foreground [&>p]:text-emphasis-important-foreground"
        case "research":
            return "bg-emphasis-research text-emphasis-research-foreground [&>p]:text-emphasis-research-foreground"
        case "primary":
            return "bg-primary text-primary-foreground [&>p]:text-primary-foreground"
        case "highlight":
            return "bg-emphasis-highlight text-emphasis-highlight-foreground [&>p]:text-emphasis-highlight-foreground"
        case "card":
            return "bg-fd-card text-fd-card-foreground [&>p]:text-fd-card-foreground"

    }
}


export const emphasisToForegroundClasses = (emphasis: Emphasis) => {
    switch (emphasis) {
        case "info":
            return "text-emphasis-info [&>p]:text-emphasis-info"
        case "error":
            return "text-emphasis-error [&>p]:text-emphasis-error"
        case "warn":
            return "text-emphasis-warn [&>p]:text-emphasis-warn"
        case "success":
            return "text-emphasis-success [&>p]:text-emphasis-success"
        case "important":
            return "text-emphasis-important [&>p]:text-emphasis-important"
        case "research":
            return "text-emphasis-research [&>p]:text-emphasis-research"
        case "primary":
            return "text-primary [&>p]:text-primary"
        case "highlight":
            return "text-emphasis-highlight [&>p]:text-emphasis-highlight"
        case "card":
            return "text-fd-card [&>p]:text-fd-card"
    }
}


/**
 * The transformation to be used in something like a `Hl`
 * component, *not* in something lke a `ColorText` component.
 */
export const emphasisBackgroundTransform = <T extends Emphasis | undefined>(defaultKey: T): EmphasisTransform => {
    return (k): string => {
        const firstKey = getFirstEmphasisKey(k) ?? defaultKey
        if (!firstKey) {
            return ""
        }
        return emphasisToBackgroundClasses(firstKey)
    }
}


/**
 * The transformation to be used in something like a `Hint`
 * or colored text component.
 */
export const emphasisForegroundTransform = <T extends Emphasis | undefined>(defaultKey: T): EmphasisTransform => {
    return (k): string => {
        const firstKey = getFirstEmphasisKey(k) ?? defaultKey
        if (!firstKey) {
            return ""
        }
        return emphasisToForegroundClasses(firstKey)
    }
}
