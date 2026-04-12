import { describe, expect, it } from "vitest";
import { faker } from "@faker-js/faker"
import { getItemsFromToc } from "./inline_toc";

describe("Successfully parses various toc inputs", () => {
    it("It successfully parses nested toc inputs", () => {
        const items = [
            {
                depth: 1,
                tab_depth: 0,
            },
            {
                depth: 2,
                tab_depth: 1,
            },
            {
                depth: 3,
                tab_depth: 2,
            },
            {
                depth: 2,
                tab_depth: 1,
            },
        ].map((n) => {
            return {
                ...n,
                content: faker.lorem.words({ min: 1, max: 10 }),
                id: faker.string.nanoid()
            }
        });
        const toc = getItemsFromToc(items, (notif) => {
            console.log("notif: ", notif);
        });

        expect(toc[0].item).toEqual(items[0])
        /* eslint-disable-next-line  -- It's a test file... let me assert things. */
        expect(toc[0].nested![0].item).toEqual(items[1])
        /* eslint-disable-next-line  -- It's a test file... let me assert things. */
        expect(toc[0].nested![0].nested![0].item).toEqual(items[2])
        /* eslint-disable-next-line  -- It's a test file... let me assert things. */
        expect(toc[0].nested![1].item).toEqual(items[3])
    });
});
