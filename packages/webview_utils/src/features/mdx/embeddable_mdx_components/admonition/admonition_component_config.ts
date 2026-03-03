import { CompletionSections, ComponentCategory, EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { snippetCompletion } from "@codemirror/autocomplete";
import { getEmphasisOptions } from "../schemas/emphasis_schema";
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { AdmonitionPropsInput, admonitionPropsSchema } from "./admonition_props_schema";
import { KeysOfType } from "@/utils/types/utility_types";


export const admonitionComponentNames = [EmbeddableComponentName.Admonition] as const

export const admonitionComponentConfig: EmbeddableComponentConfig = {
    id: EmbeddableComponentId.Admonition,
    categories: [ComponentCategory.layout, ComponentCategory.attention],
    name: admonitionComponentNames,
    schema: admonitionPropsSchema,
    desc: "A card with a colored header that can be optionally foldable. Used to draw attention to important content.",
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/admonition/admonition_component_docs.mdx",
    snippets: () => {
        const items = [
            snippetCompletion("<Admonition title=\"#{Admonition Title}\">\n\n#{body}\n\n</Admonition>", {
                section: CompletionSections.components,
                label: "admonition",
                detail: "An admonition that is not foldable.",
                type: SnippetDefaultType.function
            }),
            snippetCompletion("<Admonition title=\"#{Admonition Title}\" sidebar right>\n\n#{body}\n\n</Admonition>", {
                section: CompletionSections.components,
                label: "admonition-sidebar",
                detail: "An admonition that aligns to the side on wide viewports.",
                type: SnippetDefaultType.function
            }),
            snippetCompletion("<Admonition title=\"#{Admonition Title}\" foldable>\n\n#{body}\n\n</Admonition>", {
                section: CompletionSections.components,
                label: "admonition-foldable",
                detail: "An admonition that is foldable.",
                type: SnippetDefaultType.function
            }),
            snippetCompletion("<Admonition title=\"#{Admonition Title}\" foldable folded>\n\n#{body}\n\n</Admonition>", {
                section: CompletionSections.components,
                label: "admonition-folded",
                detail: "An admonition that is foldable and starts in a folded state.",
                type: SnippetDefaultType.function
            }),
        ]
        for (const emphasis of getEmphasisOptions()) {
            items.push(
                snippetCompletion(`<Admonition title="#{Admonition Title}" foldable ${emphasis}>\n\n#{body}\n\n</Admonition>`, {
                    section: CompletionSections.components,
                    label: `admonition-${emphasis}`,
                    detail: `A foldable Admonition of the ${emphasis} variant.`,
                    type: SnippetDefaultType.function
                }),
            )
        }
        return items
    },
    generateTestContent: async (faker, utils) => {
        return `
<Admonition ${utils.randomBooleanProperties([
            "sidebar",
            "border",
            "sidebar",
            "hideMathLabels",
            "foldable",
            "folded",
            "centerSelf",
            "centerContent"
        ] satisfies (KeysOfType<AdmonitionPropsInput, boolean>)[])} ${utils.randomEmphasis()}>
${faker.lorem.paragraphs({ min: 1, max: 10 })}
</Admonition>
`
    },
    testProps: {
        quantityScalar: 1
    }
    }
