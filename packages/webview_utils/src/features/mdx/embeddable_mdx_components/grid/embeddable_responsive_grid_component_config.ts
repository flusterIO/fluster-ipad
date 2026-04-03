import { snippetCompletion } from "@codemirror/autocomplete";
import { CompletionSections, ComponentCategory, type EmbeddableComponentConfig, SnippetDefaultType } from "../embeddable_component_config";
import { embeddableResponsiveGridPropsSchema } from "./embeddable_responsive_grid_props";
import { type EmbeddableUtilityContainerPropsInput } from "../container/embeddable_utility_container_props";
import { type KeysOfType } from "@/utils/types/utility_types";
import { SizableOption, sizableOptions } from "../schemas/sizable_props_schema";
import { EmbeddableComponentName, EmbeddableComponentId } from "../../../../core/code_gen/typeshare/conundrum";

export const gridComponentNames = [EmbeddableComponentName.Grid] as const;

export const embeddableGridComponentConfig: EmbeddableComponentConfig = {
    name: gridComponentNames,
    categories: [ComponentCategory.layout],
    desc: "A responsive grid that allows for making sure your notes align well on any screen size.",
    id: EmbeddableComponentId.Grid,
    schema: embeddableResponsiveGridPropsSchema,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/grid/grid_component_docs.mdx",
    snippets: () => {
        return [
            snippetCompletion(`<Grid none={#{1}} medium={#{2}} large={#{3}} full={#{4}}>\n\n#{Body}\n\n</Grid>`, {
                label: "grid-container",
                section: CompletionSections.components,
                type: SnippetDefaultType.function
            }),
        ]
    },
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async (faker, utils) => {
        return `
<Grid ${faker.helpers.arrayElement([SizableOption.Full, SizableOption.Medium, SizableOption.Small, SizableOption.Fit])}={2} gap="${faker.helpers.arrayElement(sizableOptions)}" ${utils.randomBooleanProperties([
            "sidebar",
            "border",
            "hideMathLabels",
            "centerSelf",
            "sidebar",
            "fit",
            "right",
        ] satisfies (KeysOfType<EmbeddableUtilityContainerPropsInput, boolean>)[])} ${utils.randomEmphasis()}>
${Array(Math.floor(Math.random() * 12)).fill(0).map((_, i) => {
            return `\n<Container centerContent>${i + 1}</Container>\n`
        }).join("\n")}
</Grid>
`
    },
    testProps: {
        quantityScalar: 0.6
    },
    isInline: false
}
