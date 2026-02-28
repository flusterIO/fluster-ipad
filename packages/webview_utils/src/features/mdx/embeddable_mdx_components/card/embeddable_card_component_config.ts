import { snippetCompletion } from "@codemirror/autocomplete";
import { CompletionSections, ComponentCategory, EmbeddableComponentConfig, EmbeddableComponentId, SnippetDefaultType } from "../embeddable_component_config";

export const embeddableCardComponentConfig: EmbeddableComponentConfig = {
    title: "Card",
    categories: [ComponentCategory.layout],
    desc: "A subtle layout with a title, an optional subtitle and a body.",
    id: EmbeddableComponentId.hl,
    snippets: () => {
        return [
            snippetCompletion(`<Card title="#{My card}" >\n\n#{Body}\n\n</Card>`, {
                label: "card",
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            }),
            snippetCompletion(`<Card title="#{My card}" desc="#{My optional description}">\n\n#{Body}\n\n</Card>`, {
                label: "card-with-desc",
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            }),
        ]
    }
}
