import { CompletionSections, ComponentCategory, EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { snippetCompletion } from "@codemirror/autocomplete";
import { getEmphasisOptions } from "../schemas/emphasis_schema";
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/fluster_core_utilities";


export const ulComponentNames = [EmbeddableComponentName.Ul, EmbeddableComponentName.UL] as const

export const ulComponentConfig: EmbeddableComponentConfig = {
    name: ulComponentNames,
    categories: [ComponentCategory.attention],
    desc: "Underline text in a variety of colors.",
    id: EmbeddableComponentId.Ul,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/ul/underline_component_docs.mdx",
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
