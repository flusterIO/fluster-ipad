import React, { type ReactNode } from "react";
import { BlogListItem } from "./blog_list_item";
import type data from "../../cdrm/cdrm.json";

export const ConundrumContentList = ({
    items,
}: {
    items: (typeof data)["files"];
}): ReactNode => {
    return (
        <div className="flex flex-col justify-center items-center ml-[264px] w-auto py-6">
            {items.map((item, i) => {
                return (
                    <BlogListItem
                        isPrimary={i === 0}
                        item={item}
                        key={item.relative_path}
                    />
                );
            })}
        </div>
    );
};

ConundrumContentList.displayName = "ConundrumContentList";
