import { heightTransformFirstChild, widthTransformFirstChild } from "../schemas/sizable_object_schema";
import { z, type ZodBoolean, type ZodEffects, type ZodOptional, type ZodString } from "zod"
import { type SizableOption, sizableOptions, sizablePropSchema, sizablePropsMapTransform } from "../schemas/sizable_props_schema";


const _data = {
    fit: z.boolean().optional(),
    full: z.boolean().optional(),
    small: z.boolean().optional(),
    smedium: z.boolean().optional(),
    medium: z.boolean().optional(),
    large: z.boolean().optional(),
    xl: z.boolean().optional(),
    xxl: z.boolean().optional(),
    none: z.boolean().optional(),
    inline: z.boolean().optional(),
    name: z.string().describe("The name of the emoji. Use the `Emoji??` docs for a list of all available emojis."),
    width: sizablePropSchema("width").optional().transform(sizablePropsMapTransform(widthTransformFirstChild)).describe("Set some custom width properties to create responsive layouts."),
    height: sizablePropSchema("height").optional().transform(sizablePropsMapTransform(heightTransformFirstChild)).describe("Set some custom height properties to create responsive layouts."),
} satisfies Record<SizableOption, ZodOptional<ZodBoolean>> & {
    name: ZodString
    inline: ZodOptional<ZodBoolean>
    width: ZodEffects<z.ZodOptional<z.ZodEnum<[SizableOption.None, SizableOption.Small, SizableOption.Smedium, SizableOption.Medium, SizableOption.Large, SizableOption.Xl, SizableOption.Xxl, SizableOption.Fit, SizableOption.Full]>>, string, SizableOption | undefined>
    height: ZodEffects<z.ZodOptional<z.ZodEnum<[SizableOption.None, SizableOption.Small, SizableOption.Smedium, SizableOption.Medium, SizableOption.Large, SizableOption.Xl, SizableOption.Xxl, SizableOption.Fit, SizableOption.Full]>>, string, SizableOption | undefined>
}


export type EmojiComponentProps = z.input<typeof emojiComponentProps>

const getEmojiContainerClasses = (d: Partial<Record<SizableOption, boolean | undefined>>): string => {
    const sizableOption = sizableOptions.find((f) => d[f])
    return ({
        fit: "w-fit h-fit",
        full: "w-full h-auto max-h-full max-w-full",
        none: "w-3 h-auto",
        small: "w-4 h-auto",
        smedium: "w-8 h-auto",
        medium: "w-16 h-auto",
        large: "w-32 h-auto",
        xl: "w-64 h-auto",
        xxl: "w-128 h-auto"
    } satisfies Record<SizableOption, string>)[sizableOption ?? "small"]
}

export const emojiComponentProps = z.object(_data).transform((c) => {
    return {
        ...c,
        containerClasses: getEmojiContainerClasses(c)
    }
});


