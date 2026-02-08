import { Completion } from "@codemirror/autocomplete"


export enum CompletionSections {
    components = "Components"
}

export enum ComponentCategory {
    layout = "Layout",
    attention = "Attention"
}

export enum EmbeddableComponentId {
    admonition,
    hl,
    ul
}

export enum SnippetDefaultType {
    class = "class",
    constant = "constant",
    enum = "enum",
    function = "function",
    interface = "interface",
    keyword = "keyword",
    method = "method",
    namespace = "namespace",
    property = "property",
    text = "text",
    type = "type",
    variable = "variable",
}


export interface EmbeddableComponentConfig {
    id: EmbeddableComponentId
    /** Categories ranked in order of priority. The first element may be used as a default value, so order matters here. */
    categories: ComponentCategory[]
    title: string;
    desc?: string;
    /** An optional path to a mdx or md file for component specific documentation and examples. Path is relative to the monorepo route. */
    docsPath?: string;
    snippets?: () => Completion[]
}
