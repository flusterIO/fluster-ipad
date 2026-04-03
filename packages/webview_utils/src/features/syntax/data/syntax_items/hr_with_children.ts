import { ParserId } from "../../../../core/code_gen/typeshare/conundrum";
import { type FlusterSyntax } from "../syntax_types";

export const hrWithChildrenSyntax: FlusterSyntax = {
    id: ParserId.HrWithChildren,
    testQuantityScalar: 0.7,
    isInline: false,
    /* eslint-disable-next-line  -- Needs to be async to match the type. */
    async testContentGenerator(f) {
        return `\n--- ${f.lorem.words({ min: 1, max: 20 })} ---\n`
    },
}
