import { CompletionSections, ComponentCategory, EmbeddableComponentConfig, EmbeddableComponentId, SnippetDefaultType } from "../embeddable_component_config";
import { snippetCompletion } from "@codemirror/autocomplete";


export const admonitionComponentConfig: EmbeddableComponentConfig = {
    id: EmbeddableComponentId.admonition,
    categories: [ComponentCategory.layout, ComponentCategory.attention],
    title: "Admonition",
    desc: "A card with a colored header that can be optionally foldable. Used to draw attention to important content.",
    snippets: () => {
        return [
            snippetCompletion("<Admonition title=\"#{Admonition Title}\">\n\n#{body}\n\n</Admonition>", {
                section: CompletionSections.components,
                label: "admonition",
                detail: "An admonition that is not foldable.",
                type: SnippetDefaultType.function
            }),
            snippetCompletion("<Admonition title=\"#{Admonition Title}\" sidebar right>\n\n#{body}\n\n</Admonition>", {
                section: CompletionSections.components,
                label: "admonition-sidebar",
                detail: "An admonition that aligns to the side on wide viewports.",
                type: SnippetDefaultType.function
            }),
            snippetCompletion("<Admonition title=\"#{Admonition Title}\" foldable>\n\n#{body}\n\n</Admonition>", {
                section: CompletionSections.components,
                label: "admonition-foldable",
                detail: "An admonition that is foldable.",
                type: SnippetDefaultType.function
            }),
            snippetCompletion("<Admonition title=\"#{Admonition Title}\" foldable folded>\n\n#{body}\n\n</Admonition>", {
                section: CompletionSections.components,
                label: "admonition-folded",
                detail: "An admonition that is foldable and starts in a folded state.",
                type: SnippetDefaultType.function
            }),
        ]
    }
    }
