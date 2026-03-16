import { CompletionSections, ComponentCategory, type EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { snippetCompletion } from "@codemirror/autocomplete";
import { getEmphasisOptions } from "../schemas/emphasis_schema";
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { hlPropsSchema } from "./hl_props_schema";


export const hlComponentNames = [EmbeddableComponentName.Hl, EmbeddableComponentName.Highlight] as const

export const hlComponentConfig: EmbeddableComponentConfig = {
    name: hlComponentNames,
    categories: [ComponentCategory.attention],
    desc: "Highlight the background of text in a variety of colors.",
    id: EmbeddableComponentId.Hl,
    schema: hlPropsSchema,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/hl/hl_component_docs.mdx",
    snippets: () => {
        return [
            snippetCompletion(`<Hl>#{content}</Hl>#{}`, {
                label: `highlight`,
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            }),
            ...getEmphasisOptions().map((c) => {
                return snippetCompletion(`<Hl ${c}>#{content}</Hl>#{}`, {
                    label: `highlight-${c}`,
                    section: CompletionSections.components,
                    type: SnippetDefaultType.function
                })
            })]
    },
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async (faker, utils) => {
        return `<Hl ${utils.randomEmphasis()}>${faker.lorem.words({ min: 1, max: 5 })}</Hl>`
    },

    testProps: {
        quantityScalar: 2
    },
    isInline: true
}
