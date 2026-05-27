import React, { type ReactNode } from "react";
import { cn } from "#/lib/cn";
import { type FileSummary } from "@conundrum/ts/ui/blog";
import { relativePathToBlogUrl } from "@conundrum/ts/pathUtils";

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
        <div
            className={cn(
                "w-full border rounded px-4 py-3 bg-fd-card",
                classes.container,
            )}
            style={{
                maxWidth: "min(1080px, calc(100% - 4rem))",
            }}
        >
            <a
                href={relativePathToBlogUrl(item.relative_path)}
                className="text-lg font-semibold"
            >
                {item.front_matter.title}
            </a>
            <div className="text-fd-card-foreground/80">
                {item.front_matter.summary}
            </div>
        </div>
    );
};

BlogListItem.displayName = "BlogListItem";
