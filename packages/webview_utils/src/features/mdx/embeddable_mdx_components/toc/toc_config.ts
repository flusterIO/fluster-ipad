import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/conundrum";
import { CompletionSections, ComponentCategory, SnippetDefaultType, type EmbeddableComponentConfig } from "../embeddable_component_config";
import { tocProps } from "./toc_props";
import { snippetCompletion } from "@codemirror/autocomplete";

export const tocComponentNames = [EmbeddableComponentName.Toc] as const;


export const tocComponentConfg: EmbeddableComponentConfig = {
    name: tocComponentNames,
    categories: [ComponentCategory.layout],
    desc: "Embed a navigable table of contents component in your note",
    id: EmbeddableComponentId.Toc,
    schema: tocProps,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/toc/toc_docs.mdx",
    snippets: () => {
        return [
            snippetCompletion(`<Toc />`, {
                label: 'table-of-contents',
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            })
        ]
    },
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async (_) => {
        return `<Toc />`
    },

    testProps: {
        quantityScalar: 0.2
    },
    isInline: false
}
