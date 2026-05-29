import React, { type ReactNode } from "react";
import { type BlogSearchParams } from "@conundrum/ts/ui/blog";
import { blogSearchParamsToQuery } from "@conundrum/ts/ui/blog-ssr";
import { getNextJsPages } from "@conundrum/ts/providers/next";
import data from "../../features/cdrm/cdrm.json";
import { ConundrumContentList } from "#/features/blog/content_list/content_list";
import { type AnyBuilderOutput } from "../../../../../packages/rust/conundrum_ts/dist/src/types/general";

interface BlogHomePageProps {
    searchParams: Promise<BlogSearchParams | undefined>;
    params: Promise<{ slug: string[] }>;
}

const BlogHomePage = async ({
    searchParams,
    params,
}: BlogHomePageProps): Promise<ReactNode> => {
    const sp = await searchParams;
    const p = await params;
    const query = blogSearchParamsToQuery(sp ?? {}, p.slug);
    const res = getNextJsPages(query, data as unknown as AnyBuilderOutput);
    return (
        <div className="min-h-screen max-h-screen h-screen overflow-y-auto overflow-x-hidden gap-y-4 flex flex-col justify-start items-center py-8">
            <ConundrumContentList
                items={
                    res.results
                        ? Array.isArray(res.results)
                            ? res.results
                            : [res.results]
                        : []
                }
            />
        </div>
    );
};

BlogHomePage.displayName = "BlogHomePage";

export default BlogHomePage;
