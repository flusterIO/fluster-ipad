import { snippetCompletion } from "@codemirror/autocomplete";
import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../../core/code_gen/typeshare/conundrum";
import { type EmbeddableComponentConfig, ComponentCategory, CompletionSections, SnippetDefaultType } from "../../embeddable_component_config";
import { equationReferencePropsSchema } from "./equation_reference_props";


export const eqRefComponentNames = [EmbeddableComponentName.EqRef] as const

export const eqRefComponentConfig: EmbeddableComponentConfig = {
    name: eqRefComponentNames,
    categories: [ComponentCategory.searchAndLinkng],
    desc: "Reference a note by it's inline index number without worrying about the structure of your note.",
    id: EmbeddableComponentId.EqRef,
    schema: equationReferencePropsSchema,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/math/equation_reference/equation_reference_docs.mdx",
    snippets: () => {
        return [
            snippetCompletion(`<EqRef id="#{mass-equivalence}"`, {
                label: `equation-reference`,
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            })
        ]
    },
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async (faker) => {
        const id = faker.word.noun({ strategy: "any-length" })
        return `$$ {#${id}}
e=mc^2
$$

My equation number is <EqRef id="${id}"/>.`
    },

    testProps: {
        quantityScalar: 0.2
    },
    isInline: false
}
