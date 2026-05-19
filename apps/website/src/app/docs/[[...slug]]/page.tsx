import React, { type ReactNode } from "react";
import { type FileSummary, type BlogSearchParams } from "@conundrum/ts/ui/blog";
import {
    SidebarPanelItem,
    blogSearchParamsToQuery,
} from "@conundrum/ts/ui/blog-ssr";
import { getNextJsPages } from "@conundrum/ts/providers/next";
import { SidebarItemPanelClient } from "#/features/blog/sidebar/item_panel_client";
import data from "../../../features/cdrm/cdrm.json";
import { ConundrumContentList } from "#/features/blog/content_list/content_list";
import { type AnyBuilderOutput } from "../../../../../../packages/rust/conundrum_ts/dist/src/types/general";

interface BlogHomePageProps {
    searchParams: Promise<BlogSearchParams>;
    params: Promise<{ slug: string[] }>;
}

const BlogHomePage = async ({
    searchParams,
    params,
}: BlogHomePageProps): Promise<ReactNode> => {
    const sp = await searchParams;
    const p = await params;
    console.log("sp: ", sp);
    console.log("p: ", p);
    const query = blogSearchParamsToQuery(sp, p.slug);
    const res = getNextJsPages(query, data as unknown as AnyBuilderOutput);
    return (
        <>
            <SidebarItemPanelClient>
                {data.files.map((f) => {
                    return (
                        <SidebarPanelItem
                            key={f.relative_path}
                            item={f as unknown as FileSummary}
                        />
                    );
                })}
            </SidebarItemPanelClient>
            {res.exactMatch ? (
                <div>Note goes here</div>
            ) : (
                <ConundrumContentList items={res.results ?? []} />
            )}
        </>
    );
};

BlogHomePage.displayName = "BlogHomePage";

export default BlogHomePage;
