import { getSizableObjectClasses, sizableObjectSchema } from "../schemas/sizable_object_schema";
import { z, type ZodBoolean, type ZodEffects, type ZodOptional, type ZodString } from "zod"
import { type SizableOption, sizableOptions } from "../schemas/sizable_props_schema";


const _data = {
    fit: z.boolean().optional().transform((c) => c ? "w-full h-auto [&_svg]:object-fit" : ""),
    full: z.boolean().optional().transform((c) => c ? "w-full h-auto [&_svg]:object-fill" : ""),
    none: z.boolean().optional().transform((c) => c ? "w-3 h-auto" : ""),
    small: z.boolean().optional().transform((c) => c ? "w-4 h-auto" : ""),
    smedium: z.boolean().optional().transform((c) => c ? "w-6 h-auto" : ""),
    medium: z.boolean().optional().transform((c) => c ? "w-12 h-auto" : ""),
    large: z.boolean().optional().transform((c) => c ? "w-32 h-auto" : ""),
    xl: z.boolean().optional().transform((c) => c ? "w-64 h-auto" : ""),
    xxl: z.boolean().optional().transform((c) => c ? "w-128 h-auto" : ""),
    inline: z.boolean().optional().transform((c) => c ? "inline-block" : "block w-full"),
    name: z.string().describe("The name of the emoji. Use the `Emoji??` docs for a list of all available emojis."),
} satisfies Record<SizableOption, ZodEffects<ZodOptional<ZodBoolean>, string, boolean | undefined>> & {
    name: ZodString
    inline: z.ZodEffects<z.ZodOptional<z.ZodBoolean>, "inline-block" | "block w-full", boolean | undefined>,
}


export type EmojiComponentProps = z.input<typeof emojiComponentProps>

const getEmojiContainerClasses = (d: Partial<Record<SizableOption, string>>): string => {
    for (const k of sizableOptions) {
        if (d[k]) {
            return d[k]
        }
    }
    return ""
}

export const emojiComponentProps = sizableObjectSchema.extend(_data).transform((c) => {
    return {
        ...c,
        containerClasses: `${getSizableObjectClasses(c)} ${getEmojiContainerClasses(c)}`
    }
});


