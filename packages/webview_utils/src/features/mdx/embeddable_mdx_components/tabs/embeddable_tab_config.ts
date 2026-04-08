import { snippetCompletion } from "@codemirror/autocomplete"
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/conundrum"
import { CompletionSections, ComponentCategory, SnippetDefaultType, type EmbeddableComponentConfig } from "../embeddable_component_config"
import { embeddableTabProps } from "./embeddable_tab_props"
import { getTabsString } from "./tab_group_component_config"

export const embeddableTabComponentNames = [EmbeddableComponentName.Tab] as const


export const embeddableTabComponentConfig: EmbeddableComponentConfig = {
    name: embeddableTabComponentNames,
    categories: [ComponentCategory.layout],
    desc: "This component works uniquely inside of a `Tabs` provider.",
    id: EmbeddableComponentId.Tab,
    schema: embeddableTabProps,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/tabs/tab_docs.mdx",
    snippets: () => {
        return [
            snippetCompletion(getTabsString(1), {
                label: `tab-item`,
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            })
        ]
    },
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async (faker) => {
        return `<Tabs>
<Tab>${faker.lorem.paragraphs({ min: 1, max: 3 })}</Tab>
</Tabs>`
    },

    testProps: {
        quantityScalar: 0.8
    },
    isInline: false
}
