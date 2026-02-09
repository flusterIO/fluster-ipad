import { CompletionSections, SnippetDefaultType } from "#/mdx/embeddable_mdx_components/embeddable_component_config";
import { snippetCompletion } from "@codemirror/autocomplete";
import { SnippetItem, SnippetStrategy } from "./snippet_types";

export const getMarkdownSnippets = (): SnippetItem[] => {
    return [
        {
            strategy: SnippetStrategy.noLeadingText,
            completion: snippetCompletion("[#{body}](#{https://google.com})", {
                label: "link",
                detail: "A markdown link",
                type: SnippetDefaultType.text,
                section: CompletionSections.markdown
            }),
        },
        {
            strategy: SnippetStrategy.noLeadingText,
            completion: snippetCompletion("```dictionary -- #{Entry Label}\n#{Body}\n```", {
                label: "dictionary-entry",
                detail: "Create a dictionary entry using the Fluster syntax.",
                type: SnippetDefaultType.text,
                section: CompletionSections.markdown
            }),
        },
    ]
}
