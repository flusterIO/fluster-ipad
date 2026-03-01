import { snippetCompletion } from "@codemirror/autocomplete";
import { CompletionSections, ComponentCategory, EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { embeddableResponsiveGridPropsSchema } from "./embeddable_responsive_grid_props";

export const gridComponentNames = [EmbeddableComponentName.Grid] as const;

export const embeddableGridComponentConfig: EmbeddableComponentConfig = {
    name: gridComponentNames,
    categories: [ComponentCategory.layout],
    desc: "A responsive grid that allows for making sure your notes align well on any screen size.",
    id: EmbeddableComponentId.Grid,
    schema: embeddableResponsiveGridPropsSchema,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/grid/grid_component_docs.mdx",
    snippets: () => {
        return [
            snippetCompletion(`<Grid none={#{1}} medium={#{2}} large={#{2}} full={#{3}}>\n\n#{Body}\n\n</Grid>`, {
                label: "grid-container",
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            }),
        ]
    }
}
