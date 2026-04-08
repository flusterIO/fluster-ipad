import { type ReactElement, type FC } from "react"

export interface EmbeddableTabItem {
    label: string | ReactElement
    /**
     * Only required when the `label` appears in more than one tab, otherwise will be automatically
     * set to the label itself.
     */
    id: string
}
