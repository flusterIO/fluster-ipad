import React, { type ReactNode } from 'react'
import { TableOfContents } from '../../features/mdx/embeddable_mdx_components/toc/table_of_contents'
import { faker } from "@faker-js/faker"
import { type MarkdownHeadingStringifiedResult } from '../../core/code_gen/typeshare/conundrum';


export const TocDevWrapper = (): ReactNode => {
    const items: MarkdownHeadingStringifiedResult[] = [];
    /* eslint-disable-next-line  -- eslint needs to catch up with rust and recognize underscores... */
    for (const _ of Array(faker.helpers.arrayElement([5, 50])).fill(0)) {
        items.push({
            tab_depth: Math.floor(Math.random() * 6),
            depth: Math.floor(Math.random() * 6),
            id: faker.word.noun(),
            content: faker.lorem.sentence({ min: 1, max: 10 })
        })
    }
    return (
        <TableOfContents toc={[{ depth: 1, tab_depth: 0, content: "Thanks man", "id": "thanks-man" }, { depth: 2, tab_depth: 1, content: "Heading two", "id": "heading-two" }, { "depth": 3, "tab_depth": 2, "content": "My other heading <Emoji name=\"heart\" small inline={true}><svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 36 36\"><path fill=\"#DD2E44\" d=\"M35.885 11.833c0-5.45-4.418-9.868-9.867-9.868-3.308 0-6.227 1.633-8.018 4.129-1.791-2.496-4.71-4.129-8.017-4.129-5.45 0-9.868 4.417-9.868 9.868 0 .772.098 1.52.266 2.241C1.751 22.587 11.216 31.568 18 34.034c6.783-2.466 16.249-11.447 17.617-19.959.17-.721.268-1.469.268-2.242z\"/></svg></Emoji>", "id": "my-other-heading-heart" }, { depth: 3, tab_depth: 3, content: "My other heading <Emoji name=\"smile\" small inline={true}><svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 36 36\"><path fill=\"#FFCC4D\" d=\"M36 18c0 9.941-8.059 18-18 18-9.94 0-18-8.059-18-18C0 8.06 8.06 0 18 0c9.941 0 18 8.06 18 18\"/><path fill=\"#664500\" d=\"M28.457 17.797c-.06-.135-1.499-3.297-4.457-3.297-2.957 0-4.397 3.162-4.457 3.297-.092.207-.032.449.145.591.175.142.426.147.61.014.012-.009 1.262-.902 3.702-.902 2.426 0 3.674.881 3.702.901.088.066.194.099.298.099.11 0 .221-.037.312-.109.177-.142.238-.386.145-.594zm-12 0c-.06-.135-1.499-3.297-4.457-3.297-2.957 0-4.397 3.162-4.457 3.297-.092.207-.032.449.144.591.176.142.427.147.61.014.013-.009 1.262-.902 3.703-.902 2.426 0 3.674.881 3.702.901.088.066.194.099.298.099.11 0 .221-.037.312-.109.178-.142.237-.386.145-.594zM18 22c-3.623 0-6.027-.422-9-1-.679-.131-2 0-2 2 0 4 4.595 9 11 9 6.404 0 11-5 11-9 0-2-1.321-2.132-2-2-2.973.578-5.377 1-9 1z\"/><path fill=\"#FFF\" d=\"M9 23s3 1 9 1 9-1 9-1-2 4-9 4-9-4-9-4z\"/></svg></Emoji>", id: "my-other-heading-smile" }]} />
    )
    /* return ( */
    /*     <TableOfContents  */
    /*         toc={items} */
    /*         toc={} */
    /*     /> */
    /* ) */
}


TocDevWrapper.displayName = "TocDevWrapper"
