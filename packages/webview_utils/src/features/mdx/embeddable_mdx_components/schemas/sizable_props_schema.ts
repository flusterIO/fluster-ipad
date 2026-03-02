import { z } from "zod";



/**
 * This sizableOptions array will be guaranteed to stay in order from smallest to largest.
 */
export const sizableOptions = [
    "none",
    "small",
    "smedium",
    "medium",
    "large",
    "xl",
    "xxl",
    "fit",
    "full"
] as const;



/**
 * An enum of all sizableOptions
 */
export const sizablePropSchema = (labelKey: string) => z.enum(sizableOptions, {
    message: `Valid ${labelKey} options are one of ${sizableOptions.map((n) => `"${n}"`).join(", ")}`
})


export type SizableOption = typeof sizableOptions[number]
export type SizableOptionRecord<T> = { [K in SizableOption]: T }
export type SizableInput = z.input<ReturnType<typeof sizablePropSchema>>
export type SizablePropsSchema = z.infer<ReturnType<typeof sizablePropSchema>>
export type SizeablePropsTransform = (value: SizableInput | undefined) => string


export const sizablePropsMapTransform = (data: Record<SizableInput, string>): SizeablePropsTransform => {
    return (val) => val ? data[val] : ""
}
