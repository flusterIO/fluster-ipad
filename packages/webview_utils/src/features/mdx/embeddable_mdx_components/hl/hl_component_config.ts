import { CompletionSections, ComponentCategory, EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { snippetCompletion } from "@codemirror/autocomplete";
import { getEmphasisOptions } from "../schemas/emphasis_schema";
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { hlPropsSchema } from "./hl_props_schema";


export const hlComponentNames = [EmbeddableComponentName.Hl, EmbeddableComponentName.Highlight] as const

export const hlComponentConfig: EmbeddableComponentConfig = {
    name: hlComponentNames,
    categories: [ComponentCategory.attention],
    desc: "Highlight; Highlight the background text in a variety of colors.",
    id: EmbeddableComponentId.Hl,
    schema: hlPropsSchema,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/hl/hl_component_docs.mdx",
    snippets: () => {
        return getEmphasisOptions().map((c) => {
            return snippetCompletion(`<Hl ${c}>#{content}</Hl>`, {
                label: `highlight-${c}`,
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            })
        })
    },
    generateTestContent: async (faker, utils) => {
        return `<Hl ${utils.randomEmphasis()}>${faker.lorem.words({ min: 1, max: 5 })}</Hl>`
    },

    testProps: {
        quantityScalar: 2
    }
}
