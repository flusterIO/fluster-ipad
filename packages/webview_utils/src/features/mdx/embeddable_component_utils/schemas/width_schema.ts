import { z } from "zod"
import { sizeSchemaOptions } from "./size_schema"

export type GetWidthSchemaProps = {
    default?: typeof sizeSchemaOptions[number]
    sizeClasses: Record<Exclude<typeof sizeSchemaOptions[number], "fit" | "full" | "none">, string>
}

export const widthSchema = (opts: GetWidthSchemaProps) => {
    let schema = z.enum(sizeSchemaOptions)
    if (opts.default) {
        /* @ts-expect-error -- Had to give up some build time type safety for some run-time type safety. */
        schema = schema.default(opts.default)
    } else {
        /* @ts-expect-error -- Had to give up some build time type safety for some run-time type safety. */
        schema = schema.optional()
    }
    return schema.transform((d): string => {
        switch (d) {
            case "small":
                return opts.sizeClasses.small;
            case "medium":
                return opts.sizeClasses.medium
            case "large":
                return opts.sizeClasses.large
            case "fit":
                return "w-fit"
            case "full":
                return "w-fit"
            case "none":
                return "w-0"
        }
    })
}


export type WidthSchema = ReturnType<typeof widthSchema>
export type WidthSchemaValueInput = z.input<WidthSchema>
export type WidthSchemaValueOutput = z.output<WidthSchema>
