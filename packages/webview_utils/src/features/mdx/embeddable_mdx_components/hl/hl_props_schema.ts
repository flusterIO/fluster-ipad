import { emphasisBackgroundTransform, emphasisSchema } from "../schemas/emphasis_schema";



export const hlPropsSchema = emphasisSchema.transform(emphasisBackgroundTransform("primary"))
