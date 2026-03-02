import { snippetCompletion } from "@codemirror/autocomplete";
import { CompletionSections, ComponentCategory, EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { embeddableCardProps } from "./embeddable_card_props";


export const cardComponentNames = [EmbeddableComponentName.Card] as const;

export const embeddableCardComponentConfig: EmbeddableComponentConfig = {
    name: cardComponentNames,
    categories: [ComponentCategory.layout],
    desc: "A subtle layout with a title, an optional subtitle and a body.",
    id: EmbeddableComponentId.Card,
    schema: embeddableCardProps,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/card/card_component_docs.mdx",
    snippets: () => {
        return [
            snippetCompletion(`<Card title="#{My card}" >\n\n#{Body}\n\n</Card>`, {
                label: "card",
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            }),
            snippetCompletion(`<Card title="#{My card}" desc="#{My optional description}">\n\n#{Body}\n\n</Card>`, {
                label: "card-with-desc",
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            }),
        ]
    },
    generateTestContent: async (faker, utils) => {
        return `
<Card title="${faker.lorem.words({ min: 1, max: 20 })}" ${utils.randomEmphasis()} ${utils.valueIfRandomProablity(`desc="${faker.lorem.sentences({ min: 1, max: 3 })}"`)}>
${faker.lorem.paragraphs({ min: 1, max: 5 })}
</Card>
`
    },
    testProps: {
        quantityScalar: 0.8
    }
}
