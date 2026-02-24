import { z } from "zod";
import { getSizableObjectClasses, sizableObjectSchema } from "../schemas/sizable_object_schema";
import { reactNodeSchema } from "../schemas/read_node_schema";
import { classRecordSchema } from "../schemas/class_record_schema";
import { emphasisSchema, getFirstEmphasisKey } from "../schemas/emphasis_schema";

export const admonitionPropsSchema = sizableObjectSchema
    .merge(emphasisSchema)
    .extend({
        title: reactNodeSchema("title"),
        folded: z.boolean({ message: "'folded' must be a boolean." }).optional(),
        foldable: z.boolean({ message: "'foldable' is a boolean." }).optional(),
        classes: classRecordSchema(["container", "body"] as const).default({})
    }).transform((c) => {
        return {
            ...c,
            type: getFirstEmphasisKey(c) ?? "primary",
            parsedContainerClasses: getSizableObjectClasses(c)
        }
    })



export type AdmonitionPropsInput = z.input<typeof admonitionPropsSchema>
export type AdmonitionPropsOutput = z.output<typeof admonitionPropsSchema>
