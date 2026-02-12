import { z } from "zod"
import { sizeSchemaOptionsNoFitFull } from "./size_schema"

export type GetMarginSchemaProps = {
    default?: typeof sizeSchemaOptionsNoFitFull[number]
    sizeClasses: Record<typeof sizeSchemaOptionsNoFitFull[number], string>
}

export const marginSchema = (opts: GetMarginSchemaProps) => {
    if (typeof opts.default === "undefined") {
        return z.enum(sizeSchemaOptionsNoFitFull).optional().transform((d): string => {
            if (d) {
                return opts.sizeClasses[d]
            } else {
                return ""
            }
        })
    } else {
        return z.enum(sizeSchemaOptionsNoFitFull).default(opts.default).transform((d): string => {
            return opts.sizeClasses[d]
        })
    }
}


export type MarginSchema = ReturnType<typeof marginSchema>
export type MarginSchemaValueInput = z.input<MarginSchema>
export type MarginSchemaValueOutput = z.output<MarginSchema>
