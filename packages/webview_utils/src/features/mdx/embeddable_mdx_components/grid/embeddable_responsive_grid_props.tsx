import { getSizableObjectClasses, sizableObjectSchema } from "../schemas/sizable_object_schema"
import { z } from "zod"
import { SizableOption, sizableOptions } from "../schemas/sizable_props_schema"



export const breakpointBySize: { [K in SizableOption]: number } = {
    none: 0,
    small: 320,
    smedium: 480,
    medium: 640,
    large: 768,
    xl: 896,
    xxl: 1024,
    fit: 0,
    full: 1080
}



export const defaultColumnsByBreakSize: { [K in SizableOption]: number | string } = {
    none: 1,
    small: 1,
    smedium: 1,
    medium: 1,
    large: 2,
    xl: 2,
    xxl: 2,
    full: 3,
    fit: ""
}


const schema = {
    none: z.number({ message: "'none' is a number that represents the number of columns when the viewport is " }).int("The 'none' field must be an integer that represents the number of columns.").or(z.string({ message: "This field can be any valid css string as well." })).optional(),
    small: z.number({ message: "'small' is a number that represents the number of columns when the viewport is " }).int("The 'small' field must be an integer that represents the number of columns.").or(z.string({ message: "This field can be any valid css string as well." })).optional(),
    smedium: z.number({ message: "'smedium' is a number that represents the number of columns when the viewport is " }).int("The 'smedium' field must be an integer that represents the number of columns.").or(z.string({ message: "This field can be any valid css string as well." })).optional(),
    medium: z.number({ message: "'medium' is a number that represents the number of columns when the viewport is " }).int("The 'medium' field must be an integer that represents the number of columns.").or(z.string({ message: "This field can be any valid css string as well." })).optional(),
    large: z.number({ message: "'large' is a number that represents the number of columns when the viewport is " }).int("The 'large' field must be an integer that represents the number of columns.").or(z.string({ message: "This field can be any valid css string as well." })).optional(),
    xl: z.number({ message: "'xl' is a number that represents the number of columns when the viewport is " }).int("The 'xl' field must be an integer that represents the number of columns.").or(z.string({ message: "This field can be any valid css string as well." })).optional(),
    xxl: z.number({ message: "'xxl' is a number that represents the number of columns when the viewport is " }).int("The 'xxl' field must be an integer that represents the number of columns.").or(z.string({ message: "This field can be any valid css string as well." })).optional(),
    full: z.number({ message: "'full' is a number that represents the number of columns when the viewport is " }).int("The 'full' field must be an integer that represents the number of columns.").or(z.string({ message: "This field can be any valid css string as well." })).optional(),
    fit: z.number({ message: "This property is pretty much here for compatibility in this case. " }).int().or(z.string({ message: "This field can be any valid css string as well." })).optional()
} satisfies { [K in SizableOption]: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodString]>> }

const columnSchema = {
    columns: z.number().int().transform((cols) => {
        return {
            none: cols,
            small: cols,
            smedium: cols,
            medium: cols,
            large: cols,
            xl: cols,
            xxl: cols,
            full: cols,
            fit: cols
        } satisfies { [K in SizableOption]: typeof cols }
    })
}


const modifiedSizableSchema = sizableObjectSchema.extend({
    centerSelf: z.boolean({ message: "'centerSelf' must be a boolean." }).optional().transform((a) => a ? "mx-auto block w-fit" : ""),
    centerContent: z.boolean({ message: "'centerContent' must be a boolean." }).optional().transform((a) => a ? "w-full place-items-center" : "").describe("Centers the content of this component's children, not the component itself."),
})


const x = modifiedSizableSchema.extend(schema)
const y = modifiedSizableSchema.extend(columnSchema)


/**
 * Returns the item at the `SizableOption` key if it exists, and if not walks down the `SizableOption` heirarchy until the next smallest value that was set. If none smaller are found, it defaults to the default value.
 */
const getSmallerItemOrDefault = (idx: number, data: { [K in SizableOption]?: number | undefined | string }, defaultValue: number | string = 1, log: boolean = false): number | string => {
    const size = sizableOptions[idx]
    const value = data[size]
    if (log) {
        console.log("size, value, idx: ", size, value, idx)
    }
    if (typeof value !== "undefined" && value !== null) {
        return value
    }

    if (idx < 1) {
        return defaultValue
    }

    return getSmallerItemOrDefault(idx - 1, data, defaultValue, log)

}


export const getSmallestSizableBreakpointByWidth = (w: number): SizableOption | undefined => {
    return sizableOptions.toReversed().find((f) => breakpointBySize[f] <= w)
}


export const getColumns = (data: { [K in SizableOption]?: number | undefined | string }): { [K in SizableOption]: number | string } => {
    return {
        none: getSmallerItemOrDefault(sizableOptions.indexOf("none"), data, defaultColumnsByBreakSize.none),
        small: getSmallerItemOrDefault(sizableOptions.indexOf("small"), data, defaultColumnsByBreakSize.small),
        smedium: getSmallerItemOrDefault(sizableOptions.indexOf("smedium"), data, defaultColumnsByBreakSize.smedium),
        medium: getSmallerItemOrDefault(sizableOptions.indexOf("medium"), data, defaultColumnsByBreakSize.medium),
        large: getSmallerItemOrDefault(sizableOptions.indexOf("large"), data, defaultColumnsByBreakSize.large),
        xl: getSmallerItemOrDefault(sizableOptions.indexOf("xl"), data, defaultColumnsByBreakSize.xl),
        xxl: getSmallerItemOrDefault(sizableOptions.indexOf("xxl"), data, defaultColumnsByBreakSize.xxl),
        full: getSmallerItemOrDefault(sizableOptions.indexOf("full"), data, defaultColumnsByBreakSize.full, true),
        // Fit won't be used directly, but applied some unique properties if the boolean is true.
        fit: defaultColumnsByBreakSize.fit
    }
}


export const embeddableResponsiveGridPropsSchema = y.or(x).transform((c) => {
    const columns = "columns" in c ? c.columns : getColumns(c)
    return {
        ...c,
        columns,
        containerClasses: getSizableObjectClasses(c)
    }
})



export type EmbeddableResponsiveGridPropsInput = z.input<typeof embeddableResponsiveGridPropsSchema>
export type EmbeddableResponsiveGridPropsOutput = z.output<typeof embeddableResponsiveGridPropsSchema>

