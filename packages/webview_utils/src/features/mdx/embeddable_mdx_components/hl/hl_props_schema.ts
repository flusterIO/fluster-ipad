import { Emphasis, emphasisBackgroundTransform, emphasisSchema } from "../schemas/emphasis_schema";
import { type z } from "zod"



export const hlPropsSchema = emphasisSchema.transform(emphasisBackgroundTransform(Emphasis.Highlight))


export type HLProps = z.input<typeof hlPropsSchema>
