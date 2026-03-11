import { type EmbeddableComponentId, type EmbeddableComponentName } from "@/code_gen/typeshare/fluster_core_utilities";
import { type Completion } from "@codemirror/autocomplete"
import { ZodAny } from "zod";
import { type AnyComponentSchema } from "./any_component_schema";
import { type Faker } from "@faker-js/faker";
import { type TestStringUtilities } from "src/development/test_string_utilities";


export enum CompletionSections {
    components = "Components",
    math = "Math",
    emoji = "Markdown: Emoji",
    markdown = "Markdown",
}

export enum ComponentCategory {
    layout = "Layout",
    attention = "Attention"
}

// export enum EmbeddableComponentId {
//     admonition = "admonition",
//     hl = "hl",
//     ul = "ul",
//     card = "card",
//     grid = "grid",
//     utlityContainer = "utlityContainer",
// }

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


export interface ComponentConfigTestData {
    /**
     * A scalar representing how many times this component is likely to occur relative to an Admonition of 1. A 'Hl' component might be higher, a plot component lower.
     */
    quantityScalar: number;
}


export interface EmbeddableComponentConfig {
    /**
     * id field is required since some components can have multiple names, even if multiple names aren't assignable to each component.
     */
    id: EmbeddableComponentId
    /** Categories ranked in order of priority. The first element may be used as a default value, so order matters here. */
    categories: ComponentCategory[]
    /**
     * name is an array to allow for multiple names to be used for each component, since there is little additional import cost.
     * @readonly to encourage exporting a constant array to be used in the get_component_map file.
     */
    name: readonly [EmbeddableComponentName, ...EmbeddableComponentName[]];
    desc: string;
    /** A path to a mdx or md file for component specific documentation and examples. Path is relative to the monorepo route. */
    docsPath: string;
    /**
     * The props schema used to parse this component's props. This is also used for documentation generation.
     */
    schema: AnyComponentSchema;
    /**
     * Generates a single item in mdx syntax with properties applied randomly.
     */
    generateTestContent: (faker: Faker, stringUtils: TestStringUtilities) => Promise<string>;
    snippets?: () => Completion[]
    testProps: ComponentConfigTestData
}
