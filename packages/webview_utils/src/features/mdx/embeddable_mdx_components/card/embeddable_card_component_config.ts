import { snippetCompletion } from "@codemirror/autocomplete";
import { CompletionSections, ComponentCategory, type EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { embeddableCardPropsSchema } from "./embeddable_card_props";
import { EmbeddableComponentName, EmbeddableComponentId } from "../../../../core/code_gen/typeshare/conundrum";


export const cardComponentNames = [EmbeddableComponentName.Card] as const;

export const embeddableCardComponentConfig: EmbeddableComponentConfig = {
    name: cardComponentNames,
    categories: [ComponentCategory.layout],
    desc: "A subtle layout with a title, an optional subtitle and a body.",
    id: EmbeddableComponentId.Card,
    schema: embeddableCardPropsSchema,
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
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async (faker, utils) => {
        return `
<Card title="${faker.lorem.words({ min: 1, max: 20 })}" ${utils.randomEmphasis()} ${utils.valueIfRandomProablity(`desc="${faker.lorem.sentences({ min: 1, max: 3 })}"`)}>
${faker.lorem.paragraphs({ min: 1, max: 5 })}
</Card>
`
    },
    testProps: {
        quantityScalar: 0.8
    },
    isInline: false
}
