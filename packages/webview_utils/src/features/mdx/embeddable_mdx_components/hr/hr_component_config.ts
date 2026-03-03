import { CompletionSections, ComponentCategory, EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { snippetCompletion } from "@codemirror/autocomplete";
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { hrPropsSchema } from "./hr_props_schema";


export const hrComponentNames = [EmbeddableComponentName.HrWithChildren] as const

export const hrComponentConfig: EmbeddableComponentConfig = {
    name: hrComponentNames,
    categories: [ComponentCategory.attention],
    desc: "Almost identical to the normal markdown based horizontal rule (the `---` syntax), but this can accept a 'content' property can be used to insert a label into the divider..",
    id: EmbeddableComponentId.Hl,
    schema: hrPropsSchema,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/hr/hr_component_docs.mdx",
    snippets: () => {
        return [snippetCompletion(`<Hl content="#{Label}" />`, {
            label: `divider-with-label`,
            section: CompletionSections.components,
            type: SnippetDefaultType.function
        })]
    },
    generateTestContent: async (faker, utils) => {
        return `<Hr ${utils.valueIfRandomProablity(`content={'${faker.lorem.words({ min: 1, max: 8 })}'}`)} />`
    },

    testProps: {
        quantityScalar: 0.2
    }
}
