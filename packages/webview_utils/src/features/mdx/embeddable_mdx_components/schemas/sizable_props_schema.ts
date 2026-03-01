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
    "full"
] as const;

export const sizablePropSchema = (labelKey: string) => z.enum(sizableOptions, {
    message: `Valid ${labelKey} options are one of ${sizableOptions.map((n) => `"${n}"`).join(", ")}`
})


export type SizableOption = typeof sizableOptions[number]
export type SizableInput = z.infer<ReturnType<typeof sizablePropSchema>>
export type SizeablePropsTransform = (value: SizableInput | undefined) => string


export const sizablePropsMapTransform = (data: Record<SizableInput, string>): SizeablePropsTransform => {
    return (val) => val ? data[val] : ""
}
