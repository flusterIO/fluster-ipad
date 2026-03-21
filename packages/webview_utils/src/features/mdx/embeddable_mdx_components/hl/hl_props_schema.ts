import { emphasisBackgroundTransform, emphasisSchema } from "../schemas/emphasis_schema";
import { type z } from "zod"



export const hlPropsSchema = emphasisSchema.transform(emphasisBackgroundTransform("highlight"))


export type HLProps = z.input<typeof hlPropsSchema>
