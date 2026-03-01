import { snippetCompletion } from "@codemirror/autocomplete";
import { CompletionSections, ComponentCategory, EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { embeddableUtiltyContainerProps } from "./embeddable_utility_container_props";

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
            snippetCompletion(`<Container centerContent width="full" height="full" padding="medium">\n\n#{Body}\n\n</Container>`, {
                label: "container-center",
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            }),
        ]
    }
}
