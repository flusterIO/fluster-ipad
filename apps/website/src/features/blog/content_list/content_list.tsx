import React, { type ReactNode } from "react";
import { BlogListItem } from "./blog_list_item";
import type data from "../../cdrm/cdrm.json";
import { type BlogFileSummary } from "../../../../../../packages/rust/conundrum_ts/dist/src/code_gen";

export const ConundrumContentList = ({
    items,
}: {
    items: BlogFileSummary[];
}): ReactNode => {
    return (
        <>
            {items.map((item, i) => {
                return (
                    <BlogListItem
                        isPrimary={i === 0}
                        item={item}
                        key={item.relative_path}
                    />
                );
            })}
        </>
    );
};

ConundrumContentList.displayName = "ConundrumContentList";
