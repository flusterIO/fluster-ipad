"use client"
import React, { type ReactNode } from "react";
import { type AnyBuilderOutput } from "../../types/general";
import { BlogItemList, type BlogItemListProps } from "./blog_item_list";
import { BlogMainSidebar } from "./blog_main_sidebar";
import { type BlogSidebarItem } from "./blog_sidebar_category";

export interface BlogTemplateProps extends Pick<
    BlogItemListProps,
    "searchParams"
> {
    data: AnyBuilderOutput;
    sidebarItems: BlogSidebarItem[]
}

export const BlogTemplate = ({
    data,
    searchParams,
    sidebarItems
}: BlogTemplateProps): ReactNode => {
    console.log("data: ", data);
    return (
        <div className="w-full h-fit min-h-screen flex flex-row gap-x-0">
            <BlogMainSidebar items={sidebarItems} />
            <BlogItemList output={data} searchParams={searchParams} />
        </div>
    );
};

BlogTemplate.displayName = "BlogTemplate";
