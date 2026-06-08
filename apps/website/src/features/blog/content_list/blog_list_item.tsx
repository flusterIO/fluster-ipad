import React, { type ReactNode } from "react";
import { cn } from "#/lib/cn";
import {
    relativePathToBlogUrl,
    subjectToUrl,
    tagsToAnyOfUrl,
    topicToUrl,
} from "@conundrum/ts/pathUtils";
import { BLOGURL } from "../constants";
import { type BlogFileSummary } from "../../../../../../packages/rust/conundrum_ts/dist/src/code_gen";

interface BlogListItemProps {
    item: BlogFileSummary;
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
        <a
            href={relativePathToBlogUrl(item.relative_path)}
            className={cn(
                "w-full border rounded px-4 py-3 bg-fd-card",
                classes.container,
            )}
            style={{
                maxWidth: "min(1080px, calc(100% - 4rem))",
            }}
        >
            <div className="w-full h-fit flex flex-col justify-start items-start">
                <div className="text-lg font-semibold">{item.front_matter.title}</div>
                <div className="w-full h-fit flex flex-wrap flex-row justify-start items-center gap-1">
                    {item.front_matter.topic ? (
                        <a
                            href={topicToUrl(item.front_matter.topic, BLOGURL)}
                            className="text-sm bg-green-600 text-background rounded-lg px-1"
                        >
                            {item.front_matter.topic}
                        </a>
                    ) : null}
                    {item.front_matter.subject ? (
                        <a
                            href={subjectToUrl(item.front_matter.subject, BLOGURL)}
                            className="text-sm bg-yellow-600 text-background rounded-lg px-1"
                        >
                            {item.front_matter.subject}
                        </a>
                    ) : null}
                    {item.tags.map((t: string) => {
                        return (
                            <a
                                href={tagsToAnyOfUrl([t], BLOGURL)}
                                key={t}
                                className="text-sm bg-primary text-primary-foreground rounded-lg px-1"
                            >
                                {t}
                            </a>
                        );
                    })}
                </div>
            </div>
            <div className="text-fd-card-foreground/80">
                {item.front_matter.summary}
            </div>
        </a>
    );
};

BlogListItem.displayName = "BlogListItem";
