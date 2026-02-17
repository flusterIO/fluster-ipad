import { Completion } from "@codemirror/autocomplete";

export enum SnippetStrategy {
    noLeadingText,
    leadingAnyCharacter
}

export type SnippetItem = {
    strategy: SnippetStrategy,
    completion: Completion
}

export interface GetSnippetProps {
    /** The base of the snippet found by the regular expression. */
    base?: string
    /** The list of keys in the user's bibliography. */
    citationKeys: string[]
    /** Required because there are like 800 of these shits. */
    includeEmojiSnippets: boolean
}
