import { ParserId } from "../../../../core/code_gen/typeshare/conundrum";
import { type FlusterSyntax } from "../syntax_types";

export const tagSyntax: FlusterSyntax = {
    id: ParserId.Tags,
    testQuantityScalar: 5,
    isInline: true,
    /* eslint-disable-next-line  -- Needs to be async to match the type. */
    testContentGenerator: async (f) => {
        return `[[#${f.lorem.words({ min: 1, max: 3 })}]]`
    }
}
