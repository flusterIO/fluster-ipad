import { ParserId } from "../../../../core/code_gen/typeshare/fluster_core_utilities";
import { type FlusterSyntax } from "../syntax_types";

export const aiParsingRequestSyntax: FlusterSyntax = {
    id: ParserId.AiTrigger,
    testQuantityScalar: 0.5,
    isInline: false,
    /* eslint-disable-next-line  -- Need to be async to match the type. */
    async testContentGenerator(f) {
        return `
\`\`\`fluster-ai 
${f.lorem.sentences({ min: 1, max: 5 })}
\`\`\`
`
    },
}
