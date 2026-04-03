import { ParserId } from "../../../../core/code_gen/typeshare/conundrum";
import { type FlusterSyntax } from "../syntax_types";

export const dictionarySyntax: FlusterSyntax = {
    id: ParserId.Dictionary,
    testQuantityScalar: 1,
    isInline: false,
    /*eslint-disable-next-line  -- Needs to be async to match the type */
    async testContentGenerator(f) {
        return `
\`\`\`dictionary -- ${f.lorem.words({ min: 1, max: 15 })}

${f.lorem.paragraphs({ min: 1, max: 5 })}

\`\`\`
`
    },
}
