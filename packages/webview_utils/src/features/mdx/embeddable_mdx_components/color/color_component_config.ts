import { CompletionSections, ComponentCategory, type EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { snippetCompletion } from "@codemirror/autocomplete";
import { EmbeddableComponentName, EmbeddableComponentId } from "../../../../core/code_gen/typeshare/conundrum";


export const colorComponentNames = [EmbeddableComponentName.Color] as const

export const colorComponentConfig: EmbeddableComponentConfig = {
    name: colorComponentNames,
    categories: [ComponentCategory.media],
    desc: "Great for UI & Design workflows, create and label colors or groups of colors either inline or block level.",
    id: EmbeddableComponentId.Color,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/color/color_component_docs.mdx",
    snippets: () => {
        return [snippetCompletion(`<Color color="#{royalblue}" />`, {
            label: `color`,
            section: CompletionSections.components,
            type: SnippetDefaultType.function
        }),
        snippetCompletion(`<Color color="#{royalblue}" inline />`, {
            label: `color-inline`,
            section: CompletionSections.components,
            type: SnippetDefaultType.function
        })
        ]
    },
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async (faker, utils) => {
        return `<Hr ${utils.valueIfRandomProablity(`content={'${faker.lorem.words({ min: 1, max: 8 })}'}`)} />`
    },

    testProps: {
        quantityScalar: 0.2
    },
    isInline: false
}
