import { getSizableObjectClasses, sizableObjectSchema } from "../schemas/sizable_object_schema"
import { z } from "zod"
import { SizableOption, sizableOptions, sizablePropSchema, sizablePropsMapTransform } from "../schemas/sizable_props_schema"
import { Mutable } from "../../../../core/utils/types/utility_types"



export const breakpointBySize: { [K in SizableOption]: number } = {
    none: 0,
    small: 320,
    smedium: 480,
    medium: 640,
    large: 768,
    xl: 896,
    xxl: 1024,
    /**
     * Have to carve out a unique path for `fit` in some cases because the breakpoint is out of order.
     */
    fit: 0,
    full: 1080
}



/**
 * Used for sorting
 */
export const sizableMagnitudeMap: { [K in SizableOption]: number } = {
    none: 0,
    small: 1,
    smedium: 2,
    medium: 3,
    large: 4,
    xl: 5,
    xxl: 6,
    fit: 7,
    full: 8,
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
    fit: z.boolean({ message: "If true while 'responsive' is set this will cause items to expand to fill the empty space." }).optional(),
    responsive: z.boolean({ message: "If set to a number, this is the minimum width of each grid column." }).or(sizablePropSchema("responsive")).optional(),
} satisfies { [K in Exclude<SizableOption, "fit" | "responsive">]: z.ZodOptional<z.ZodUnion<[z.ZodNumber, z.ZodString]>> } & { fit: z.ZodOptional<z.ZodBoolean>, responsive: z.ZodOptional<z.ZodUnion<[z.ZodBoolean, z.ZodEnum<Mutable<typeof sizableOptions>>]>> }

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
    fit: z.boolean({ message: "If true while 'responsive' is set this will cause items to expand to fill the empty space." }).optional(),
    responsive: z.boolean({ message: "If set to a number, this is the minimum width of each grid column." }).or(sizablePropSchema("responsive")).optional(),
    gap: sizablePropSchema("gap").default("medium").transform(sizablePropsMapTransform({
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
})


const x = modifiedSizableSchema.extend(schema)
const y = modifiedSizableSchema.extend(columnSchema)


/**
 * Returns the item at the `SizableOption` key if it exists, and if not walks down the `SizableOption` heirarchy until the next smallest value that was set. If none smaller are found, it defaults to the default value.
 */
const getSmallerItemOrDefault = (idx: number, data: { [K in SizableOption]?: number | undefined | string }, defaultValue: number | string = 1): number | string => {
    const size = sizableOptions[idx]
    const value = data[size]
    if (typeof value !== "undefined" && value !== null) {
        return value
    }

    if (idx < 1) {
        return defaultValue
    }

    return getSmallerItemOrDefault(idx - 1, data, defaultValue)

}


export const getSmallestSizableBreakpointByWidth = (w: number): SizableOption | undefined => {
    return sizableOptions.toReversed().filter((n) => n !== "fit").find((f) => {
        return breakpointBySize[f] <= w
    })
}


export const getColumns = (data: { [K in Exclude<SizableOption, "fit">]?: number | undefined | string }): { [K in SizableOption]: number | string } => {
    return {
        none: getSmallerItemOrDefault(sizableOptions.indexOf("none"), data, defaultColumnsByBreakSize.none),
        small: getSmallerItemOrDefault(sizableOptions.indexOf("small"), data, defaultColumnsByBreakSize.small),
        smedium: getSmallerItemOrDefault(sizableOptions.indexOf("smedium"), data, defaultColumnsByBreakSize.smedium),
        medium: getSmallerItemOrDefault(sizableOptions.indexOf("medium"), data, defaultColumnsByBreakSize.medium),
        large: getSmallerItemOrDefault(sizableOptions.indexOf("large"), data, defaultColumnsByBreakSize.large),
        xl: getSmallerItemOrDefault(sizableOptions.indexOf("xl"), data, defaultColumnsByBreakSize.xl),
        xxl: getSmallerItemOrDefault(sizableOptions.indexOf("xxl"), data, defaultColumnsByBreakSize.xxl),
        full: getSmallerItemOrDefault(sizableOptions.indexOf("full"), data, defaultColumnsByBreakSize.full),
        // Fit won't be used directly, but applied some unique properties if the boolean is true.
        fit: defaultColumnsByBreakSize.fit
    }
}


export const embeddableResponsivieGridPropsSchemaUnion = y.or(x)


const responsiveSizableGridMap: { [K in SizableOption]: number } = {
    none: 0,           // No minimum (will collapse)
    small: 120,        // Very narrow items (e.g., small cards/icons)
    smedium: 180,      // Compact items
    medium: 240,       // Standard card size
    large: 320,        // Substantial items (mobile-width)
    xl: 480,           // Half-page width on many screens
    xxl: 640,          // Large hero or feature sections
    fit: 800,          // Likely to force 1 column on tablets
    full: 1200,
}

const getResponsiveClasses = (data: z.infer<typeof embeddableResponsivieGridPropsSchemaUnion>,): string | undefined => {
    const sizing = data.fit ? "auto-fit" : "auto-fill"
    if (typeof data.responsive === "number") {
        return `repeat(${sizing}, minmax(${data.responsive}px, 1fr))`
    }
    if (typeof data.responsive === "string" && data.responsive in responsiveSizableGridMap) {
        return `repeat(${sizing}, minmax(${responsiveSizableGridMap[data.responsive]}px, 1fr))`
    }
    return ""
}


export const embeddableResponsiveGridPropsSchema = embeddableResponsivieGridPropsSchemaUnion.transform((c) => {
    const columns = "columns" in c ? c.columns : getColumns(c)
    return {
        ...c,
        columns,
        containerClasses: getSizableObjectClasses(c),
        responsiveTemplateColumns: typeof c.responsive !== "undefined" ? getResponsiveClasses(c) : undefined
    }
})



export type EmbeddableResponsiveGridPropsInput = z.input<typeof embeddableResponsiveGridPropsSchema>
export type EmbeddableResponsiveGridPropsOutput = z.output<typeof embeddableResponsiveGridPropsSchema>

