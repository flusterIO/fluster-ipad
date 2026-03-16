import { ParserId } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { type FlusterSyntax } from "../syntax_types";
import { embeddableComponentConfigs } from "../../../mdx/embeddable_mdx_components/component_configs";

export const docsSyntax: FlusterSyntax = {
    id: ParserId.Documentation,
    testQuantityScalar: 0.5,
    isInline: false,
    /* eslint-disable-next-line  -- Needs to be async to match the type. */
    async testContentGenerator(f) {
        return `\n${f.helpers.arrayElement(embeddableComponentConfigs.map((n) => n.name[0]))}??\n`
    },
}
