import { embeddableComponentConfigs } from "#/mdx/embeddable_mdx_components/component_configs";
import { getMathSnippets } from "./math_snippets";
import { getMarkdownSnippets } from "./markdown_snippets";
import { SnippetItem, SnippetStrategy } from "./snippet_types";

export const getFlusterSnippets = (base?: string): SnippetItem[] => {
    let items: SnippetItem[] = [
        ...getMathSnippets(base),
        ...getMarkdownSnippets()
    ]
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
