import { CompletionSections, ComponentCategory, EmbeddableComponentConfig, EmbeddableComponentId, SnippetDefaultType } from "../embeddable_component_config";
import { flusterColorKeys } from "../embeddable_component_types/color_key";
import { snippetCompletion } from "@codemirror/autocomplete";


export const ulComponentConfig: EmbeddableComponentConfig = {
    title: "Ul",
    categories: [ComponentCategory.attention],
    desc: "Underline; Underline text in a variety of colors.",
    id: EmbeddableComponentId.hl,
    snippets: () => {
        return flusterColorKeys.map((c) => {
            return snippetCompletion(`<Ul ${c}>#{content}</Ul>`, {
                label: `underline-${c}`,
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            })
        })
    }
}
