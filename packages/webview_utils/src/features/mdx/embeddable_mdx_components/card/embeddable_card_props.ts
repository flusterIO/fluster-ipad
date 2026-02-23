import { z } from "zod";

export const embeddableCardProps = z.object({

})


export type EmbeddableCardProps = z.infer<typeof embeddableCardProps>
export type EmbeddableCardPropsInput = z.input<typeof embeddableCardProps>
