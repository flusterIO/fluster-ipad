import { z } from "zod";
import { sizablePropSchema, sizablePropsMapTransform } from "./sizable_props_schema";

const _sizableObjectSchema = {
    right: z.boolean("'right' must be a boolean.").optional().transform((a) => a ? "float-right" : ""),
    left: z.boolean("'left' must be a boolean.").optional().transform((a) => a ? "float-left" : ""),
    center: z.boolean("'center' must be a boolean.").optional().transform((a) => a ? "mx-auto" : ""),
    text: sizablePropSchema("text").optional().transform(sizablePropsMapTransform({
        none: "text-xs",
        small: "text-sm",
        smedium: "text-base",
        medium: "text-lg",
        large: "text-xl",
        xl: "text-2xl",
        xxl: "text-4xl",
        full: "text-7xl",
    })),
    width: sizablePropSchema("width").optional().transform(sizablePropsMapTransform({
        none: "w-0",
        small: "max-w-xs",
        smedium: "max-w-sm",
        medium: "max-w-md",
        large: "max-w-lg",
        xl: "max-w-2xl",
        xxl: "max-w-4xl",
        full: "w-full",
    })),
    height: sizablePropSchema("height").optional().transform(sizablePropsMapTransform({
        none: "h-0",
        small: "h-24",
        smedium: "h-32",
        medium: "h-48",
        large: "h-64",
        xl: "h-96",
        xxl: "h-screen",
        full: "h-full",
    })),
    margin: sizablePropSchema("margin").optional().transform(sizablePropsMapTransform({
        none: "m-0",
        small: "m-2",
        smedium: "m-3",
        medium: "m-4",
        large: "m-6",
        xl: "m-8",
        xxl: "m-12",
        full: "m-16",
    })),
    marginLeft: sizablePropSchema("marginLeft").optional().transform(sizablePropsMapTransform({
        none: "ml-0",
        small: "ml-2",
        smedium: "ml-3",
        medium: "ml-4",
        large: "ml-6",
        xl: "ml-8",
        xxl: "ml-12",
        full: "ml-16",
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
    })),
    paddingLeft: sizablePropSchema("paddingLeft").optional().transform(sizablePropsMapTransform({
        none: "pl-0",
        small: "pl-2",
        smedium: "pl-3",
        medium: "pl-4",
        large: "pl-6",
        xl: "pl-8",
        xxl: "pl-12",
        full: "pl-16",
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
    })),
}


export const sizableObjectSchema = z.object(_sizableObjectSchema);

export type SizableObject = z.infer<typeof sizableObjectSchema>

export const getSizalbeObjectClasses = (data: z.output<typeof sizableObjectSchema>) => {
    const classes: string[] = []

    Object.keys(_sizableObjectSchema).forEach((k) => {
        classes.push(data[k as keyof typeof _sizableObjectSchema])
    })


    return classes.join(" ")
}
