// import { snippetCompletion } from "@codemirror/autocomplete"
// import { type SnippetItem, SnippetStrategy } from "./snippet_types"
// import { CompletionSections, SnippetDefaultType } from "../../../../mdx/embeddable_mdx_components/embeddable_component_config"

export enum EmojiCategory {
    people = "People",
    nature = "Nature",
    objects = "Object",
    places = "Places",
    symbols = "Symbols"
}



export interface EmojiSnippetDataItem {
    value: string
    category: EmojiCategory
}


