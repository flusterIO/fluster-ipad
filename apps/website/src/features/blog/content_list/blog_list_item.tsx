import React, { type ReactNode } from "react";
import { cn } from "#/lib/cn";
import { type FileSummary } from "@conundrum/ts/ui/blog";

interface BlogListItemProps {
    item: FileSummary;
    /**
     * True if the blog item is the primary blog item and should occupy the full width.
     */
    isPrimary: boolean;
    classes?: {
        container?: string;
    };
}

export const BlogListItem = ({
    item,
    classes = {},
}: BlogListItemProps): ReactNode => {
    if (
        !item.front_matter?.title ||
        !item.front_matter.summary ||
        !item.front_matter.user_defined_id
    ) {
        console.error(
            "Found a note without valid front matter. Make sure to fix this ace.",
        );
        return null;
    }
    return (
        <div className={cn("w-full", classes.container)}>
            <a href={`/blog?id=${item.front_matter.user_defined_id}`}>
                {item.front_matter.title}
            </a>
            <div>{item.front_matter.summary}</div>
        </div>
    );
};

BlogListItem.displayName = "BlogListItem";
