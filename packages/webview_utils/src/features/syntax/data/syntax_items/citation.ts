import { ParserId } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { type FlusterSyntax } from "../syntax_types";

export const citationSyntax: FlusterSyntax = {
    id: ParserId.Citations,
    testQuantityScalar: 0.8,
    isInline: true,
    /* eslint-disable-next-line  -- Need to be async to match the type. */
    async testContentGenerator(f) {
        return `[[cite:${f.lorem.word()}]]`
    },
}
