import { z } from "zod";

export const childrenSchema = z.any().describe("The 'children' property is a unique property. You _can_ insert itusing the same syntax as other properties, but _unlike_ other properties, children can be inserted directly inside of the component. Most developers choose to represent certain properties as a 'child' when the content logically goes 'inside' of the component.")


export const withChildrenRequiredSchema = z.object({
    children: childrenSchema.refine((a) => typeof a !== "undefined")
})

export const withChildrenSchema = z.object({
    children: childrenSchema.optional()
})
