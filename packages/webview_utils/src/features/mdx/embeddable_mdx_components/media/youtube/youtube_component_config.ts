import { EmbeddableComponentId, EmbeddableComponentName } from "@/code_gen/typeshare/conundrum"
import { CompletionSections, ComponentCategory, type EmbeddableComponentConfig, SnippetDefaultType } from "../../embeddable_component_config"
import { snippetCompletion } from "@codemirror/autocomplete"

export const youtubeComponentNames = [EmbeddableComponentName.Youtube] as const


export const youtubeComponentConfig: EmbeddableComponentConfig = {
    name: youtubeComponentNames,
    categories: [ComponentCategory.media],
    desc: "Great for UI & Design workflows, create and label colors or groups of colors either inline or block level.",
    id: EmbeddableComponentId.Youtube,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/media/youtube/youtube_component_docs.mdx",
    snippets: () => {
        return [
            snippetCompletion(`<Youtube url="#{videoUrl}" />`, {
                label: `youtube`,
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            })
        ]
    },
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async (faker, utils) => {
        return `<Youtube url="https://youtu.be/-7DvvDhyyNA" />`
    },

    testProps: {
        quantityScalar: 0.2
    },
    isInline: false
}
