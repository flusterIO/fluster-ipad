import { CompletionSections, ComponentCategory, EmbeddableComponentConfig, EmbeddableComponentId, SnippetDefaultType } from "../embeddable_component_config";
import { flusterColorKeys } from "../embeddable_component_types/color_key";
import { snippetCompletion } from "@codemirror/autocomplete";


export const hlComponentConfig: EmbeddableComponentConfig = {
    title: "Hl",
    categories: [ComponentCategory.attention],
    desc: "Highlight; Highlight the background text in a variety of colors.",
    id: EmbeddableComponentId.hl,
    snippets: () => {
        return flusterColorKeys.map((c) => {
            return snippetCompletion(`<Hl ${c}>#{content}</Hl>`, {
                label: `highlight-${c}`,
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            })
        })
    }
}
