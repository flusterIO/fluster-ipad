import { CompletionSections, ComponentCategory, type EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { snippetCompletion } from "@codemirror/autocomplete";
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { hrPropsSchema } from "./hr_props_schema";


export const hrComponentNames = [EmbeddableComponentName.HrWithChildren] as const

export const hrComponentConfig: EmbeddableComponentConfig = {
    name: hrComponentNames,
    categories: [ComponentCategory.attention],
    desc: "Almost identical to the normal markdown based horizontal rule (the `---` syntax), but this can accept a 'content' property can be used to insert a label into the divider.",
    id: EmbeddableComponentId.HrWithChildren,
    schema: hrPropsSchema,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/hr/hr_component_docs.mdx",
    snippets: () => {
        return [snippetCompletion(`<Hr> #{Section} </Hr>`, {
            label: `divider-with-label`,
            section: CompletionSections.components,
            type: SnippetDefaultType.function
        })]
    },
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async (faker, utils) => {
        return `<Hr ${utils.valueIfRandomProablity(`content={'${faker.lorem.words({ min: 1, max: 8 })}'}`)} />`
    },

    testProps: {
        quantityScalar: 0.2
    }
}
