import { CompletionSections, ComponentCategory, EmbeddableComponentConfig, EmbeddableComponentId, SnippetDefaultType } from "../embeddable_component_config";
import { flusterColorKeys } from "../embeddable_component_types/color_key";
import { snippetCompletion } from "@codemirror/autocomplete";
import { getEmphasisOptions } from "../schemas/emphasis_schema";


export const ulComponentConfig: EmbeddableComponentConfig = {
    title: "Ul",
    categories: [ComponentCategory.attention],
    desc: "Underline; Underline text in a variety of colors.",
    id: EmbeddableComponentId.hl,
    snippets: () => {
        const items = getEmphasisOptions().map((c) => {
            return snippetCompletion(`<Ul ${c}>#{content}</Ul>`, {
                label: `underline-${c}`,
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            })
        })

        items.push(
            snippetCompletion(`<Ul thick primary>#{content}</Ul>`, {
                label: `underline-thick`,
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            })

        )
        return items
    }
}
