import { describe, it, expect } from "vitest";
import { faker } from "@faker-js/faker"
import { SiaString } from "./sia_string";

describe("SiaString class serializes and deserializes strings properly.", () => {
    it("Seralizes string to and from a uint-8-array and returns the original value.", () => {
        const s = faker.lorem.paragraphs({ min: 10, max: 100 })
        const output = new SiaString(s).serialize()
        const outputString = SiaString.deserialize(output)
        expect(outputString).toEqual(s)
    });
});
