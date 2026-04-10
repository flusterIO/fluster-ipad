import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/conundrum"
import { CompletionSections, ComponentCategory, type EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { emojiComponentProps } from "./emoji_component_props";
import { snippetCompletion } from "@codemirror/autocomplete";

export const emojiComponentNames = [
    EmbeddableComponentName.Emoji
] as const;

export const emojiComponentConfig: EmbeddableComponentConfig = {
    name: emojiComponentNames,
    categories: [ComponentCategory.attention],
    desc: "Embed emoji's either with the standard `:smile:` notation or with the sizable Emoji component.",
    id: EmbeddableComponentId.Emoji,
    schema: emojiComponentProps,
    docsPath: "docs/in_content_docs/emoji-docs.mdx",
    snippets: () => {
        return [
            snippetCompletion(`<Emoji name="#{smile}" />`, {
                label: `emoji`,
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            })
        ]
    },
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async (_) => {
        return `<Emoji name="smile" />`
    },

    testProps: {
        quantityScalar: 0.2
    },
    isInline: true
}
