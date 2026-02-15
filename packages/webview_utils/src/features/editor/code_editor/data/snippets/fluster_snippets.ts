import { embeddableComponentConfigs } from "#/mdx/embeddable_mdx_components/component_configs";
import { getMarkdownSnippets } from "./markdown_snippets";
import { GetSnippetProps, SnippetItem, SnippetStrategy } from "./snippet_types";


export const getFlusterSnippets = (props: GetSnippetProps): SnippetItem[] => {
    let items: SnippetItem[] = getMarkdownSnippets(props);
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
