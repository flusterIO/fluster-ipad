import React, { type ReactNode } from "react";
import { type AnyNoteOutput } from "../../types/general";
import consola from "consola";

interface BlogItemProps {
    item: AnyNoteOutput;
}

export const BlogItem = ({ item }: BlogItemProps): ReactNode => {
    if (!item.front_matter?.title) {
        consola.error(`Found a note without a title! ${item.relative_path}`);
    }
    if (!item.front_matter?.summary) {
        consola.error(
            "Found a blog entry without a summary. You can give the user a better experience by providing a short summary. Here's the path: ",
            item.relative_path,
        );
    }
    return (
        <div className="w-full max-w-[min(1080px,90vw)] rounded border bg-fd-card px-4 py-3">
            <div className="text-lg font-bold">
                {item.front_matter?.title ?? "No title found."}
            </div>
            <div>{item.front_matter?.summary}</div>
        </div>
    );
};

BlogItem.displayName = "BlogItem";
