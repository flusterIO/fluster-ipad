import { z } from "zod";
import { reactNodeSchema } from "../schemas/read_node_schema";

export const embeddableTabProps = z.object({
    label: reactNodeSchema("label"),
    labelString: z.string().describe("The `labelString` property should have been automatically applied from your `label` property. Have you applied a label?"),
    id: z.string({ message: "The label field is required if the id field already appears in this list of tabs." }).optional()
})


export type EmbeddableTabItemProps = z.infer<typeof embeddableTabProps>
