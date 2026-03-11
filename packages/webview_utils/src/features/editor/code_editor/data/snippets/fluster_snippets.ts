import { embeddableComponentConfigs } from "#/mdx/embeddable_mdx_components/component_configs";
import { getDocumentationSnippets } from "./documentation_snippets";
import { getEmojiSnippets } from "./emoji_snippets";
import { getMarkdownSnippets } from "./markdown_snippets";
import { type GetSnippetProps, type SnippetItem, SnippetStrategy } from "./snippet_types";


export const getFlusterSnippets = (props: GetSnippetProps): SnippetItem[] => {
    let items: SnippetItem[] = [
        ...getMarkdownSnippets(props),
        ...getDocumentationSnippets()
    ]
    if (props.includeEmojiSnippets) {
        items = [
            ...items,
            ...getEmojiSnippets()
        ]
    }
    embeddableComponentConfigs.map((c) => {
        if (typeof c.snippets === "function") {
            const res = c.snippets()
            items = items.concat(...res.map((r) => {
                return {
                    strategy: SnippetStrategy.noLeadingText,
                    completion: r
                }
            }))
        }
    })
    return items
}
