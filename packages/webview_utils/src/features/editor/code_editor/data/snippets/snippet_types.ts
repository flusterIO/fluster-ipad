import { Completion } from "@codemirror/autocomplete";

export enum SnippetStrategy {
    noLeadingText,
    leadingAnyCharacter
}


export type SnippetItem = {
    strategy: SnippetStrategy,
    completion: Completion
}
