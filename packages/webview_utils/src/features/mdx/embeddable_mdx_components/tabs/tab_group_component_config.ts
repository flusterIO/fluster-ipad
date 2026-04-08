import { snippetCompletion } from "@codemirror/autocomplete"
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/conundrum"
import { CompletionSections, ComponentCategory, SnippetDefaultType, type EmbeddableComponentConfig } from "../embeddable_component_config"
import { tabGroupComponentProps } from "./tab_group_props"

export const tabGroupComponentNames = [EmbeddableComponentName.Tabs] as const

export const getTabsString = (count: number): string => {
    const s = []
    for (let i = 0; i < count; i++) {
        s.push(`    <Tab label="#{Tab Label${i === 0 ? "" : ` ${i + 1}`}}">
        #{Conundrum Content}
    </Tab>`)

    }
    return s.join("\n")
}

export const tabGroupComponentConfig: EmbeddableComponentConfig = {
    name: tabGroupComponentNames,
    categories: [ComponentCategory.layout],
    desc: "Create a set of interchangable tabs.",
    id: EmbeddableComponentId.Tabs,
    schema: tabGroupComponentProps,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/tabs/tab_group_docs.mdx",
    snippets: () => {
        const s = []
        for (let i = 1; i <= 5; i++) {
            s.push(
                snippetCompletion(`<Tabs> 
${getTabsString(i)}
</Tabs>`, {
                    label: `tabs-${i}`,
                    section: CompletionSections.components,
                    type: SnippetDefaultType.function
                })
            )
        }
        return s
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
