import { CompletionSections, SnippetDefaultType } from "#/mdx/embeddable_mdx_components/embeddable_component_config";
import { snippetCompletion } from "@codemirror/autocomplete";
import { GetSnippetProps, SnippetItem, SnippetStrategy } from "./snippet_types";

const genTableOfColumns = (columns: number): string => {
    let s = "|";
    Array(columns).fill(0).forEach((_, i) => {
        s += ` #{col-${i + 1}} |`
    })
    s += "\n|"
    Array(columns).fill(0).forEach(() => {
        s += " ----- |"
    })
    s += "\n"
    return s
}


export const getMarkdownSnippets = (props: GetSnippetProps): SnippetItem[] => {
    return [
        {
            strategy: SnippetStrategy.noLeadingText,
            completion: snippetCompletion("[#{body}](#{https://google.com})", {
                label: "link",
                detail: "A markdown link",
                type: SnippetDefaultType.text,
                section: CompletionSections.markdown,
                boost: 50
            }),
        },
        {
            strategy: SnippetStrategy.noLeadingText,
            completion: snippetCompletion("```dictionary -- #{Entry Label}\n#{Body}\n```", {
                label: "dictionary-entry",
                detail: "Create a dictionary entry using the Fluster syntax.",
                type: SnippetDefaultType.text,
                section: CompletionSections.markdown,
                boost: 50
            }),
        },
        {
            strategy: SnippetStrategy.noLeadingText,
            completion: snippetCompletion("- [ ] #{Task details}", {
                label: "task-incomplete",
                detail: "Creates a task that is incomplete",
                type: SnippetDefaultType.text,
                section: CompletionSections.markdown,
                boost: 50
            }),
        },
        {
            strategy: SnippetStrategy.noLeadingText,
            completion: snippetCompletion("- [x] #{Task details}", {
                label: "task-complete",
                detail: "Creates a task that is complete.",
                type: SnippetDefaultType.text,
                boost: 50,
                section: CompletionSections.markdown
            }),
        },
        {
            strategy: SnippetStrategy.noLeadingText,
            completion: snippetCompletion("[[##{ToDo}]]", {
                label: "tag",
                detail: "Creates a tag",
                type: SnippetDefaultType.text,
                boost: 50,
                section: CompletionSections.markdown
            }),
        },
        {
            strategy: SnippetStrategy.noLeadingText,
            completion: snippetCompletion("$#{e=mc^2}$", {
                label: "inline-math",
                type: SnippetDefaultType.variable,
                detail: "Open an inline math block",
                boost: 50,
                section: CompletionSections.markdown
            }),
        },
        {
            strategy: SnippetStrategy.noLeadingText,
            completion: snippetCompletion("$$\n#{e=mc^2}\n$$", {
                label: "math",
                type: SnippetDefaultType.variable,
                detail: "Open a math block",
                boost: 50,
                section: CompletionSections.markdown
            }),

        },
        {
            strategy: SnippetStrategy.noLeadingText,
            completion: snippetCompletion(`\`\`\`mermaid
---
title: #{Simple sample}
---
#{stateDiagram-v2}
\`\`\` `, {
                label: "mermaid",
                type: SnippetDefaultType.variable,
                detail: "Creates a mermaid code block",
                boost: 50,
                section: CompletionSections.markdown
            })
        },
        ...props.citationKeys.filter((c) => c.trim().length).map((citationKey) => {
            return {
                strategy: SnippetStrategy.noLeadingText,
                completion: snippetCompletion(`[[cite:${citationKey}]]`, {
                    label: `Citation: ${citationKey}`,
                    section: CompletionSections.markdown,
                    boost: 50,
                    type: SnippetDefaultType.text
                })
            } satisfies SnippetItem
        }),
        ...[2, 3, 4, 5].map((columns) => {
            return {
                strategy: SnippetStrategy.noLeadingText,
                completion: snippetCompletion(genTableOfColumns(columns), {
                    label: `Table (${columns} columns)`,
                    section: CompletionSections.markdown,
                    boost: 50,
                    type: SnippetDefaultType.text
                })
            } satisfies SnippetItem
        })
    ]
}
