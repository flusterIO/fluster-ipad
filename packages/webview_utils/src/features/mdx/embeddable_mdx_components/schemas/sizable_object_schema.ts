import { z } from "zod";
import { SizableOptionRecord, sizablePropSchema, sizablePropsMapTransform, sizablePropsOrBooleanTransform, sizablePropsSchemaOrBool } from "./sizable_props_schema";
import { componentNeverProperty } from "./component_never_property";



export const defaultWidthTransform: SizableOptionRecord<string> = {
    none: "w-full @[768px]/mdx:hidden",
    small: "w-full @[450px]/mdx:w-[320px]",
    smedium: "w-full @[550px]/mdx:w-[384px]]",
    medium: "w-full @[650px]/mdx:w-[448px]",
    large: "w-full @[768px]/mdx:w-[576px]",
    xl: "w-full @[1080px]/mdx:w-[672px]",
    xxl: "w-full @[1080px]/mdx:w-[896px]",
    full: "w-full",
    fit: "w-fit"
}

export const defaultHeightTransform: SizableOptionRecord<string> = {
    none: "h-fit",
    small: "h-24",
    smedium: "h-32",
    medium: "h-48",
    large: "h-64",
    xl: "h-96",
    xxl: "h-screen",
    full: "h-full",
    fit: "h-fit"
}

export const _sizableObjectSchema = {
    hideMathLabels: z.boolean().optional().transform((a) => a ? "hide-math-labels" : "").describe("Hides the MathJax labels in all child components."),
    right: z.boolean({ message: "'right' must be a boolean." }).optional().transform((a) => a ? "float-right ml-4 mr-0" : "").describe("'Floats' the component to the right. This is often combined with `width` or the `sidebar` property to create sidebar layouts."),
    left: z.boolean({ message: "'left' must be a boolean." }).optional().transform((a) => a ? "float-left mr-4 ml-0" : "").describe("'Floats' the component to the left. This is often combined with `width` or the `sidebar` property to create sidebar layouts."),
    sidebar: z.boolean({ message: "'sidebar' is a boolean property." }).optional().transform((a) => a ? "w-full min-w-full @[768px]/mdx:w-1/3 @[768px]/mdx:min-w-[33%]" : "").describe('A utility property that sets a responsive max-width to create sidebar like layouts on large screens while allowing for full-width when the window is smaller.'),
    center: componentNeverProperty("You're probably looking for either 'centerContent' to center this component's children, or 'centerSelf' to attempt to center this component itself.").transform(() => "").describe("This will _never_ work. This is put here only as an error trigger to notify users of `centerSelf` and `centerContent`"),
    centerSelf: z.boolean({ message: "'centerSelf' must be a boolean." }).optional().transform((a) => a ? "mx-auto block" : ""),
    centerContent: z.boolean({ message: "'centerContent' must be a boolean." }).optional().transform((a) => a ? "flex flex-col justify-center items-center text-center [&>p]:text-center" : "").describe("Centers the content of this component's children, not the component itself."),
    border: z.boolean().optional().transform((k) => k ? "border" : "").describe("Add a small, muted border to the object."),
    rounded: sizablePropsSchemaOrBool("rounded").optional().transform(sizablePropsOrBooleanTransform({
        none: "rounded-none",
        small: "rounded-sm",
        smedium: "rounded-sm",
        medium: "rounded-medium",
        large: "rounded-lg",
        xl: "rounded-2xl",
        xxl: "rounded-4xl",
        full: "rounded-full",
        fit: "rounded-[100%]"
    })).describe("Rounds the corners of the container. Use `rounded=\"full\"` or `rounded=\"fit\"` to create a completely circular component, useful for some image layouts."),
    text: sizablePropSchema("text").optional().transform(sizablePropsMapTransform({
        none: "text-xs",
        small: "text-sm",
        smedium: "text-base",
        medium: "text-lg",
        large: "text-xl",
        xl: "text-2xl",
        xxl: "text-4xl",
        full: "text-7xl",
        fit: "w-full text-center"
    })).describe("Change the text content of the container's children. Beware though, some edge cases might not respond as expected."),
    width: sizablePropSchema("width").optional().transform(sizablePropsMapTransform(defaultWidthTransform)).describe("Set some custom width properties to create responsive layouts."),
    height: sizablePropSchema("height").optional().transform(sizablePropsMapTransform(defaultHeightTransform)).describe("Set some custom height properties to create responsive layouts."),
    margin: sizablePropSchema("margin").optional().transform(sizablePropsMapTransform({
        none: "m-0",
        small: "m-2",
        smedium: "m-3",
        medium: "m-4",
        large: "m-6",
        xl: "m-8",
        xxl: "m-12",
        full: "m-16",
        fit: "m-0"
    })).describe("Add some padding around the **outside** of an object. If you are looking to create space on the _inside_ of an object you are looking for `padding`."),
    marginLeft: sizablePropSchema("marginLeft").optional().transform(sizablePropsMapTransform({
        none: "ml-0",
        small: "ml-2",
        smedium: "ml-3",
        medium: "ml-4",
        large: "ml-6",
        xl: "ml-8",
        xxl: "ml-12",
        full: "ml-16",
        fit: "ml-auto"
    })),
    marginRight: sizablePropSchema("marginRight").optional().transform(sizablePropsMapTransform({
        none: "mr-0",
        small: "mr-2",
        smedium: "mr-3",
        medium: "mr-4",
        large: "mr-6",
        xl: "mr-8",
        xxl: "mr-12",
        full: "mr-16",
        fit: "mr-auto"
    })),
    marginTop: sizablePropSchema("marginTop").optional().transform(sizablePropsMapTransform({
        none: "mt-0",
        small: "mt-2",
        smedium: "mt-3",
        medium: "mt-4",
        large: "mt-6",
        xl: "mt-8",
        xxl: "mt-12",
        full: "mt-16",
        fit: "mt-auto"
    })),
    marginBottom: sizablePropSchema("marginBottom").optional().transform(sizablePropsMapTransform({
        none: "mb-0",
        small: "mb-2",
        smedium: "mb-3",
        medium: "mb-4",
        large: "mb-6",
        xl: "mb-8",
        xxl: "mb-12",
        full: "mb-16",
        fit: "mb-auto"
    })),
    marginX: sizablePropSchema("marginX").optional().transform(sizablePropsMapTransform({
        none: "mx-0",
        small: "mx-2",
        smedium: "mx-3",
        medium: "mx-4",
        large: "mx-6",
        xl: "mx-8",
        xxl: "mx-12",
        full: "mx-16",
        fit: "mx-auto"
    })),
    marginY: sizablePropSchema("marginY").optional().transform(sizablePropsMapTransform({
        none: "my-0",
        small: "my-2",
        smedium: "my-3",
        medium: "my-4",
        large: "my-6",
        xl: "my-8",
        xxl: "my-12",
        full: "my-16",
        fit: "my-auto"
    })),
    padding: sizablePropSchema("padding").optional().transform(sizablePropsMapTransform({
        none: "p-0",
        small: "p-2",
        smedium: "p-3",
        medium: "p-4",
        large: "p-6",
        xl: "p-8",
        xxl: "p-12",
        full: "p-16",
        fit: "p-0"
    })).describe("Create padding on the _inside_ of an object. If you are trying to create space _around_ an object. you are probably looking for `margin`."),
    paddingLeft: sizablePropSchema("paddingLeft").optional().transform(sizablePropsMapTransform({
        none: "pl-0",
        small: "pl-2",
        smedium: "pl-3",
        medium: "pl-4",
        large: "pl-6",
        xl: "pl-8",
        xxl: "pl-12",
        full: "pl-16",
        fit: "pl-0"
    })),
    paddingRight: sizablePropSchema("paddingRight").optional().transform(sizablePropsMapTransform({
        none: "pr-0",
        small: "pr-2",
        smedium: "pr-3",
        medium: "pr-4",
        large: "pr-6",
        xl: "pr-8",
        xxl: "pr-12",
        full: "pr-16",
        fit: "pr-0"
    })),
    paddingTop: sizablePropSchema("paddingTop").optional().transform(sizablePropsMapTransform({
        none: "pt-0",
        small: "pt-2",
        smedium: "pt-3",
        medium: "pt-4",
        large: "pt-6",
        xl: "pt-8",
        xxl: "pt-12",
        full: "pt-16",
        fit: "pt-0"
    })),
    paddingBottom: sizablePropSchema("paddingBottom").optional().transform(sizablePropsMapTransform({
        none: "pb-0",
        small: "pb-2",
        smedium: "pb-3",
        medium: "pb-4",
        large: "pb-6",
        xl: "pb-8",
        xxl: "pb-12",
        full: "pb-16",
        fit: "pb-0"
    })),
    paddingX: sizablePropSchema("paddingX").optional().transform(sizablePropsMapTransform({
        none: "px-0",
        small: "px-2",
        smedium: "px-3",
        medium: "px-4",
        large: "px-6",
        xl: "px-8",
        xxl: "px-12",
        full: "px-16",
        fit: "px-0"
    })),
    paddingY: sizablePropSchema("paddingY").optional().transform(sizablePropsMapTransform({
        none: "py-0",
        small: "py-2",
        smedium: "py-3",
        medium: "py-4",
        large: "py-6",
        xl: "py-8",
        xxl: "py-12",
        full: "py-16",
        fit: "py-0"
    })),
    gap: sizablePropSchema("paddingY").optional().transform(sizablePropsMapTransform({
        none: "gap-0",
        small: "gap-2",
        smedium: "gap-3",
        medium: "gap-4",
        large: "gap-6",
        xl: "gap-8",
        xxl: "gap-12",
        full: "gap-16",
        fit: "gap-0"
    })).describe("When in Grid mode or in some other select layouts, this property create a gap between _all_ children."),
    gapX: sizablePropSchema("paddingY").optional().transform(sizablePropsMapTransform({
        none: "gap-x-0",
        small: "gap-x-2",
        smedium: "gap-x-3",
        medium: "gap-x-4",
        large: "gap-x-6",
        xl: "gap-x-8",
        xxl: "gap-x-12",
        full: "gap-x-16",
        fit: "gap-0"
    })),
    gapY: sizablePropSchema("paddingY").optional().transform(sizablePropsMapTransform({
        none: "gap-y-0",
        small: "gap-y-2",
        smedium: "gap-y-3",
        medium: "gap-y-4",
        large: "gap-y-6",
        xl: "gap-y-8",
        xxl: "gap-y-12",
        full: "gap-y-16",
        fit: "gap-0"
    })),
}


export const sizableObjectSchema = z.object(_sizableObjectSchema);

export type SizableObject = z.infer<typeof sizableObjectSchema>

export const getSizableObjectClasses = (data: { [K in keyof SizableObject]: string }, ignoreSizableKeys: (keyof typeof _sizableObjectSchema)[] | null = null) => {
    const classes: string[] = []

    Object.keys(_sizableObjectSchema).forEach((k) => {
        if (!ignoreSizableKeys || (ignoreSizableKeys && !ignoreSizableKeys.includes(k as keyof typeof _sizableObjectSchema))) {
            classes.push(data[k as keyof typeof _sizableObjectSchema])
        }
    })


    return classes.filter((a) => a.length).join(" ")
}
