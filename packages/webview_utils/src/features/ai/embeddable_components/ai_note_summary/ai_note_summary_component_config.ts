import { ComponentCategory, type EmbeddableComponentConfig } from "#/mdx/embeddable_mdx_components/embeddable_component_config";
import { EmbeddableComponentId, EmbeddableComponentName } from "@/code_gen/typeshare/fluster_core_utilities";
import { aiNoteSummaryProps } from "./ai_note_summary_props";

export const noteSummaryComponentNames = [EmbeddableComponentName.AINoteSummary] as const

export const ulComponentConfig: EmbeddableComponentConfig = {
    name: noteSummaryComponentNames,
    categories: [ComponentCategory.ai],
    desc: "Generate a summary of your note automatically and optionally apply it to your note's search result item.",
    id: EmbeddableComponentId.AINoteSummary,
    schema: aiNoteSummaryProps,
    docsPath: "packages/webview_utils/src/features/ai/embeddable_components/ai_note_summary/ai_note_summary_docs.mdx",
    snippets: () => {
        // const items = [
        //     snippetCompletion(`<Ul>#{content}</Ul>`, {
        //         label: `underline`,
        //         section: CompletionSections.components,
        //         type: SnippetDefaultType.function
        //     }),
        //     ...getEmphasisOptions().map((c) => {
        //         return snippetCompletion(`<Ul ${c}>#{content}</Ul>`, {
        //             label: `underline-${c}`,
        //             section: CompletionSections.components,
        //             type: SnippetDefaultType.function
        //         })
        //     })]
        // items.push(
        //     snippetCompletion(`<Ul thick highlight>#{content}</Ul>`, {
        //         label: `underline-thick`,
        //         section: CompletionSections.components,
        //         type: SnippetDefaultType.function
        //     })

        // )
        return []
    },
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async () => {
        return `<AINoteSummary />`
    },

    testProps: {
        quantityScalar: 0.1
    },
    isInline: true
}
