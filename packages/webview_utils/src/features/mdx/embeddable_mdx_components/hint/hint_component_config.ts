import { CompletionSections, ComponentCategory, EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { snippetCompletion } from "@codemirror/autocomplete";
import { getEmphasisOptions } from "../schemas/emphasis_schema";
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { embeddableHintComponentPropsSchema } from "./hint_props_schema";


export const embeddableHintComponentNames = [EmbeddableComponentName.Hint] as const

export const embeddableHintComponentConfig: EmbeddableComponentConfig = {
    name: embeddableHintComponentNames,
    categories: [ComponentCategory.attention],
    desc: "Add some subtle, colored text to draw attention to smaller content.",
    id: EmbeddableComponentId.Hint,
    schema: embeddableHintComponentPropsSchema,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/hint/embeddable_hint_component_docs.mdx",
    snippets: () => {
        return getEmphasisOptions().map((c) => {
            return snippetCompletion(`<Hint ${c}>\n#{content}\n</Hint>#{}`, {
                label: `hint-${c}`,
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
