import { ParserId } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { type FlusterSyntax } from "../syntax_types";

export const noteLinkSyntax: FlusterSyntax = {
    testQuantityScalar: 3,
    isInline: true,
    id: ParserId.NoteLink,
    /* eslint-disable-next-line  -- Needs to be async to match the type */
    async testContentGenerator(f) {
        return `[${f.lorem.words({ min: 1, max: 5 })}](note:${f.lorem.word()})`
    },
}
