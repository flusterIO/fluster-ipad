import { CompletionSections, SnippetDefaultType } from "#/mdx/embeddable_mdx_components/embeddable_component_config";
import { snippetCompletion } from "@codemirror/autocomplete";
import { SnippetItem, SnippetStrategy } from "./snippet_types";
import { EmbeddableComponentName, InContentDocumentationId } from "@/code_gen/typeshare/fluster_core_utilities";


export const getDocumentationSnippets = (): SnippetItem[] => {
    const items: SnippetItem[] = []
    for (const k of Object.values(EmbeddableComponentName)) {
        items.push({
            strategy: SnippetStrategy.noLeadingText,
            completion: snippetCompletion(`${k}??`, {
                label: `docs-${k}`,
                detail: `Documentation for the ${k} component`,
                type: SnippetDefaultType.text,
                section: CompletionSections.markdown,
                boost: 50
            })
        })
    }
    for (const internalDocs of Object.values(InContentDocumentationId)) {
        items.push({
            strategy: SnippetStrategy.noLeadingText,
            completion: snippetCompletion(`${internalDocs}??`, {
                label: `docs-${internalDocs}`,
                detail: `General documentation`,
                type: SnippetDefaultType.text,
                section: CompletionSections.markdown,
                boost: 50
            }),

        })
    }
    return items
}
