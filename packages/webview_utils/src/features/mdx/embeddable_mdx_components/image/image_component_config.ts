import { EmbeddableComponentId, EmbeddableComponentName } from "../../../../core/code_gen/typeshare/conundrum";
import { type EmbeddableComponentConfig, ComponentCategory } from "../embeddable_component_config";
import { imageComponentPropsSchema } from "./image_component_props_schema";


export const imageComponentNames = [EmbeddableComponentName.Image] as const


export const imageComponentConfig: EmbeddableComponentConfig = {
    name: imageComponentNames,
    categories: [ComponentCategory.media],
    desc: "Insert an image and size it accordingly.",
    id: EmbeddableComponentId.Image,
    schema: imageComponentPropsSchema,
    docsPath: "packages/webview_utils/src/features/mdx/embeddable_mdx_components/image/image_component_docs.mdx",
    snippets: () => {
        return [
        ]
    },
    /* eslint-disable-next-line  -- I know there's no await, but it needs to match the interface. */
    generateTestContent: async () => {
        return ``
    },

    testProps: {
        quantityScalar: 0.2
    },
    isInline: false
}
