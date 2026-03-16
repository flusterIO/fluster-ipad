import { snippetCompletion } from "@codemirror/autocomplete";
import { CompletionSections, ComponentCategory, type EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { type EmbeddableUtilityContainerPropsInput, embeddableUtiltyContainerProps } from "./embeddable_utility_container_props";
import { type KeysOfType } from "@/utils/types/utility_types";

export const utilityContainerComponentNames = [EmbeddableComponentName.UtlityContainer] as const;

export const embeddableUtilityContainerComponentConfig: EmbeddableComponentConfig = {
    name: utilityContainerComponentNames,
    categories: [ComponentCategory.layout],
    desc: "A simple, unstyled utility component that can be used to apply arbitrary styles to content.",
    id: EmbeddableComponentId.UtlityContainer,
    schema: embeddableUtiltyContainerProps,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/container/utility_container_docs.mdx",
    snippets: () => {
        return [
            snippetCompletion(`<Container centerContent width="full" height="fit" padding="medium">\n\n#{Body}\n\n</Container>`, {
                label: "container-center",
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            }),
        ]
    },
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async (faker, utils) => {
        return `
<Container ${utils.randomBooleanProperties([
            "sidebar",
            "border",
            "hideMathLabels",
            "centerSelf",
            "sidebar",
            "fit",
            "right",
        ] satisfies (KeysOfType<EmbeddableUtilityContainerPropsInput, boolean>)[])} ${utils.randomEmphasis()}>
${faker.lorem.paragraphs({ min: 1, max: 10 })}
</Container>
`
    },
    testProps: {
        quantityScalar: 3
    },
    isInline: false
}
