import { CompletionSections, ComponentCategory, type EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { snippetCompletion } from "@codemirror/autocomplete";
import { getEmphasisOptions } from "../schemas/emphasis_schema";
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { ulPropsSchema } from "./ul_props_schema";


export const ulComponentNames = [EmbeddableComponentName.Ul, EmbeddableComponentName.Underline] as const

export const ulComponentConfig: EmbeddableComponentConfig = {
    name: ulComponentNames,
    categories: [ComponentCategory.attention],
    desc: "Underline text in a variety of colors.",
    id: EmbeddableComponentId.Ul,
    schema: ulPropsSchema,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/ul/underline_component_docs.mdx",
    snippets: () => {
        const items = [
            snippetCompletion(`<Ul>#{content}</Ul>`, {
                label: `underline`,
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            }),
            ...getEmphasisOptions().map((c) => {
                return snippetCompletion(`<Ul ${c}>#{content}</Ul>`, {
                    label: `underline-${c}`,
                    section: CompletionSections.components,
                    type: SnippetDefaultType.function
                })
            })]

        items.push(
            snippetCompletion(`<Ul thick highlight>#{content}</Ul>`, {
                label: `underline-thick`,
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            })

        )
        return items
    },
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async (faker, utils) => {
        return `<Ul ${utils.randomEmphasis()}>${faker.lorem.words({ min: 1, max: 5 })}</Ul>`
    },

    testProps: {
        quantityScalar: 2
    },
    isInline: true
}
