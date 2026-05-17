import React, { useMemo, type ReactNode } from "react";
import { BlogSidebarCategory, type BlogSidebarItem } from "./blog_sidebar_category";

interface BlogMainSidebarProps {
    items: BlogSidebarItem[];
}

type SidebarCategoryMap = Record<string, BlogSidebarItem[]>;

export const BlogMainSidebar = ({ items }: BlogMainSidebarProps): ReactNode => {
    const itemMap = useMemo((): SidebarCategoryMap => {
        const data: SidebarCategoryMap = {};
        for (const k of items) {
            /* eslint-disable-next-line  -- Now eslint's complaining about shit that's not even true.. */
            if (!data[k.category]) {
                data[k.category] = [];
            }
            data[k.category].push(k);
        }
        return data;
    }, [items]);
    return (
        <div className="w-1/3 max-w-[min(90vw,350px)] flex flex-col justify-start items-start gap-y-0">
            {Object.keys(itemMap).map((item) => {
                return <BlogSidebarCategory category={item} items={itemMap[item]} />;
            })}
        </div>
    );
};

BlogMainSidebar.displayName = "BlogMainSidebar";
