import { z } from "zod"
import { GetWidthSchemaProps, widthSchema } from "./width_schema"
import { GetMarginSchemaProps, marginSchema } from "./margin_schema"


export interface GetSizableSchemaProps {
    width: GetWidthSchemaProps;
    margin: GetMarginSchemaProps
    /** The classes applied for the properties that are applied as booleans. */
    booleanClasses: {
        right: string;
        left: string;
        center: string
    }
}

const sizableSchemaKeys = [
    "width",
    "margin",
    "right",
    "left",
    "center"
] as const

const sizeableSchemaData = (opts: GetSizableSchemaProps) => {
    return {
        width: widthSchema(opts.width),
        margin: marginSchema(opts.margin),
        right: z.boolean().default(false).describe("Float the component to the right in large views.").transform((c) => {
            if (c) {
                return opts.booleanClasses.right
            }
        }),
        left: z.boolean().default(false).describe("Float the component to the right in large views.").transform((c) => {
            if (c) {
                return opts.booleanClasses.left
            }
        }),
        center: z.boolean().default(false).describe("Apply some common classes to attempt to center the component.").transform((c) => {
            if (c) {
                return opts.booleanClasses.center
            }
        })
    }
}

export const sizeableSchema = (opts: GetSizableSchemaProps) => {
    return z.object(sizeableSchemaData(opts))
}


export type SizableSchema = ReturnType<typeof sizeableSchema>
export type SizableSchemaKey = keyof ReturnType<typeof sizeableSchemaData>
export type SizableSchemaInput = z.input<SizableSchema>
export type SizableSchemaOutput = z.output<SizableSchema>


export const getSizeableSchemaClasses = (data: SizableSchemaOutput): string => {
    const s: string[] = []
    for (const k of sizableSchemaKeys) {
        if (typeof data[k] === "string") {
            s.push(data[k])
        }
    }
    return s.join(" ")
}
