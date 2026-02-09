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
    base?: string
    /** The list of keys in the user's bibliography. */
    citationKeys: string[]
}
