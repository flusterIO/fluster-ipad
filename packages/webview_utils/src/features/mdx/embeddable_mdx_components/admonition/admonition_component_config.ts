import { CompletionSections, ComponentCategory, EmbeddableComponentConfig, EmbeddableComponentId, SnippetDefaultType } from "../embeddable_component_config";
import { snippetCompletion } from "@codemirror/autocomplete";
import { getEmphasisOptions } from "../schemas/emphasis_schema";


export const admonitionComponentConfig: EmbeddableComponentConfig = {
    id: EmbeddableComponentId.admonition,
    categories: [ComponentCategory.layout, ComponentCategory.attention],
    title: "Admonition",
    desc: "A card with a colored header that can be optionally foldable. Used to draw attention to important content.",
    snippets: () => {
        let items = [
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
        for (const emphasis of getEmphasisOptions()) {
            items.push(
                snippetCompletion(`<Admonition title=\"#{Admonition Title}\" foldable ${emphasis}>\n\n#{body}\n\n</Admonition>`, {
                    section: CompletionSections.components,
                    label: `admonition-${emphasis}`,
                    detail: `A foldable Admonition of the ${emphasis} variant.`,
                    type: SnippetDefaultType.function
                }),
            )
        }
        return items
    }
    }
