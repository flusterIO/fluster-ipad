"use client";
import React, { useMemo, type ReactNode } from "react";
import { type AnyBuilderOutput } from "../../types/general";
import { BlogItemList, type BlogItemListProps } from "./blog_item_list";
import { BlogMainSidebar } from "./blog_main_sidebar";
import { type BlogSidebarItem } from "./blog_sidebar_category";
import { BlogProviderProvider } from "./state/blog_provider";

export interface BlogTemplateProps extends Pick<
    BlogItemListProps,
    "searchParams"
> {
    data: AnyBuilderOutput;
    sidebarItems: BlogSidebarItem[];
    banner?: ReactNode;
    slug: string[];
}

export const BlogTemplate = ({
    data,
    searchParams,
    sidebarItems,
    banner,
}: BlogTemplateProps): ReactNode => {
    return (
        <BlogProviderProvider>
            <div className="w-full h-fit min-h-screen flex flex-row gap-x-0">
                <BlogMainSidebar banner={banner} items={sidebarItems} />
                <BlogItemList output={data} searchParams={searchParams} />
            </div>
        </BlogProviderProvider>
    );
};

BlogTemplate.displayName = "BlogTemplate";
