import { embeddableComponentConfigs } from "#/mdx/embeddable_mdx_components/component_configs";
import { type Completion } from "@codemirror/autocomplete";

export const getFlusterSnippets = (): Completion[] => {
    let items: Completion[] = []
    embeddableComponentConfigs.map((c) => {
        if (typeof c.snippets === "function") {
            const res = c.snippets()
            items = items.concat(...res)
        }
    })
    return items
}
