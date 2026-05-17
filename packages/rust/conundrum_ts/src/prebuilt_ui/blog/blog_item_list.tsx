import React, { type ReactNode } from "react";
import { type AnyBuilderOutput } from "../../types/general";
import { BlogItem } from "./blog_item";
import { type BlogSearchParams } from "./types";
import { blogSearchParamsToQuery } from "./methods/blog_search_params_to_query";
import { getNextJsPage } from "../../providers/next";
import { NoBlogEntriesFound } from "./empty/no_entries_found";

export interface BlogItemListProps {
    output: AnyBuilderOutput;
    searchParams?: BlogSearchParams;
}

export const BlogItemList = ({
    output,
    searchParams,
}: BlogItemListProps): ReactNode => {
    const notes = searchParams
        ? getNextJsPage(blogSearchParamsToQuery(searchParams), output)
        : output.files;
    console.log("notes: ", notes);
    return (
        <div className="w-full max-w-[min(1080px,90vw)] flex flex-col justify-center items-center gap-y-4">
            {notes ? (
                notes.map((m) => {
                    return <BlogItem item={m} key={m.relative_path} />;
                })
            ) : (
                <NoBlogEntriesFound />
            )}
        </div>
    );
};

BlogItemList.displayName = "BlogItemList";
