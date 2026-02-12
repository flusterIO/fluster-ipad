import { z } from "zod"

export const sizeSchemaOptions = [
    "small",
    "medium",
    "large",
    "none",
    "fit",
    "full"
] as const


export type GetSizeSchemaOptionsProps = {
    default: typeof sizeSchemaOptions[number]
    sizeClasses: Record<typeof sizeSchemaOptions[number], string>
};


export const sizeSchema = (opts: GetSizeSchemaOptionsProps) => {
    return z.enum(sizeSchemaOptions).default(opts.default).transform((t): string => {
        return opts.sizeClasses[t]
    })
}



export const sizeSchemaOptionsNoFitFull = [
    "small",
    "medium",
    "large",
    "none"
] as const


export type GetSizeSchemaOptionsPropsNoFitFull = {
    default: typeof sizeSchemaOptions[number]
    sizeClasses: Record<typeof sizeSchemaOptions[number], string>
};


export const sizeSchemaNoFitFull = (opts: GetSizeSchemaOptionsProps) => {
    return z.enum(sizeSchemaOptions).default(opts.default).transform((t): string => {
        return opts.sizeClasses[t]
    })
} 
