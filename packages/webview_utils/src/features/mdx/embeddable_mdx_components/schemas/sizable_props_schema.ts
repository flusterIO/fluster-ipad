import { z } from "zod";
import { SizableOption } from "../../../../core/code_gen/typeshare/fluster_core_utilities"



/**
 * This sizableOptions array will be guaranteed to stay in order from smallest to largest.
 */
export const sizableOptions = [
    SizableOption.None,
    SizableOption.Small,
    SizableOption.Smedium,
    SizableOption.Medium,
    SizableOption.Large,
    SizableOption.Xl,
    SizableOption.Xxl,
    SizableOption.Fit,
    SizableOption.Full,
] as const satisfies SizableOption[];



/**
 * An enum of all sizableOptions
 */
export const sizablePropSchema = (labelKey: string) => z.enum(sizableOptions, {
    message: `Valid ${labelKey} options are one of ${sizableOptions.map((n) => `"${n}"`).join(", ")}`
})


export const sizablePropsSchemaOrBool = (labelKey: string) => z.enum(
    sizableOptions, {
    message: `Valid ${labelKey} options are one of ${sizableOptions.map((n) => `"${n}"`).join(", ")}`
}
).or(z.boolean({ message: `The ${labelKey} property can also be a boolean.` }))


export {
    SizableOption
}
export type SizableOptionRecord<T> = { [K in SizableOption]: T }
export type SizableInput = z.input<ReturnType<typeof sizablePropSchema>>
export type SizablePropsSchema = z.infer<ReturnType<typeof sizablePropSchema>>
export type SizableOrBooleanInput = z.input<ReturnType<typeof sizablePropsSchemaOrBool>>
export type SizableOrBooleanPropsSchema = z.infer<ReturnType<typeof sizablePropsSchemaOrBool>>
export type SizeablePropsTransform = (value: SizableInput | undefined) => string
export type SizeableOrBooleanPropsTransform = (value: SizableOrBooleanInput | undefined) => string


export const sizablePropsMapTransform = (data: Record<SizableInput, string>): SizeablePropsTransform => {
    return (val) => val ? data[val] : ""
}


/**
 * @param defaultValue - The css class to be applied when a user provides true.
 */
export const sizablePropsOrBooleanTransform = (data: Record<SizableInput, string>,
    defaultValue: string): SizeableOrBooleanPropsTransform => {
    return (val): string => {
        if (val === true) {
            return defaultValue
        }
        if (typeof val === "string" && val in data) {
            return data[val]
        }
        return ""
    }
}

