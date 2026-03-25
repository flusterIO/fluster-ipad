import { sizableObjectSchema } from "#/mdx/embeddable_mdx_components/schemas/sizable_object_schema";
import { z } from "zod";

export const aiNoteSummaryProps = sizableObjectSchema.extend({
    generate: z.boolean().optional().describe("Force the component to regenerate the data. Beware however that notes with multiple AI components may take significant time to load when this property is set to true indefinitely."),
    apply: z.boolean().optional().describe("Apply the output of this component to the note's persisted summary. This will show the generated summary in your search results."),
    applyButton: z.boolean().or(z.literal("auto")).default("auto").describe("A boolean or 'auto' property that indicates whether or not the 'Apply' button should be shown, allowing you to click to apply the generated content to your note as opposed to re-rendering the component and possibly generating new data to provide the 'apply' property. By default, this button will be shown whenever the generated summary does not match your persisted summary.")

})
