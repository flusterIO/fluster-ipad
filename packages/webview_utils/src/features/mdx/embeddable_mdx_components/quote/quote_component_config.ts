import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/conundrum";
import { type EmbeddableComponentConfig, ComponentCategory } from "../embeddable_component_config";
import { quotePropsSchema } from "./quote_props_schema";


export const quoteComponentNames = [EmbeddableComponentName.Quote] as const


export const quoteComponentConfig: EmbeddableComponentConfig = {
    name: quoteComponentNames,
    categories: [ComponentCategory.searchAndLinkng],
    desc: "Insert a quote from a significant person, book, or another source.",
    id: EmbeddableComponentId.Quote,
    schema: quotePropsSchema,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/quote/quote_component_documentation.mdx",
    snippets: () => {
        return [
        ]
    },
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async (faker) => {
        return `<Quote source="${faker.lorem.words({ min: 1, max: 3 })}">\n${faker.lorem.sentences({ min: 1, max: 3 })}\n</Quote>`
    },

    testProps: {
        quantityScalar: 0.2
    },
    isInline: false
}
