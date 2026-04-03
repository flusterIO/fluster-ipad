import { type ParserId } from "@/code_gen/typeshare/conundrum";
import { type Faker } from "@faker-js/faker";


export interface FlusterSyntax {
    testContentGenerator: (f: Faker) => Promise<string>;
    /**
     * A number representing the likelihood of this component occurring. Let the estimated probability of a dictionary entry be 1. A 'tag' would be > 1, as would a note link, but others may be lower.
     */
    testQuantityScalar: number
    /**
     * Does this syntax occur within a paragraph like tag or citation, 
     * or is it block level like a admonition.
     */
    isInline: boolean;
    id: ParserId;
}
