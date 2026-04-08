import { Emphasis, emphasisBackgroundTransform, emphasisSchema } from "../schemas/emphasis_schema";
import { getSizableObjectClasses, sizableObjectSchema } from "../schemas/sizable_object_schema";
import { type z } from "zod"

export const tabGroupComponentProps = sizableObjectSchema.merge(emphasisSchema).transform((c) => {
    return {
        ...c,
        containerClasses: getSizableObjectClasses(c),
        buttonClasses: emphasisBackgroundTransform(Emphasis.Card)(c)
    }
});


export type EmbeddableTabGroupProps = z.input<typeof tabGroupComponentProps>;
export type EmbeddableTabGroupPropsOutput = z.output<typeof tabGroupComponentProps>;
