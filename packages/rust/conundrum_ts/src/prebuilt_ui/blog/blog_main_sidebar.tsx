"use client";
import React, { useMemo, useState, type ReactNode } from "react";
import {
    BlogSidebarCategory,
    type BlogSidebarItem,
} from "./blog_sidebar_category";
import { motion } from "framer-motion";
import { BlogSidebarTitlebar } from "./blog_sidebar_titlebar";
import { useBlogProviderContext } from "./state/blog_provider";

interface BlogMainSidebarProps {
    items: BlogSidebarItem[];
    banner?: ReactNode;
}

type SidebarCategoryMap = Record<string, BlogSidebarItem[]>;

export const BlogMainSidebar = ({
    items,
    banner,
}: BlogMainSidebarProps): ReactNode => {
    const { sidebar } = useBlogProviderContext();
    const itemMap = useMemo(() => {
        const itemMap: SidebarCategoryMap = {};
        for (const k of items) {
            if (!(k.category in itemMap)) {
                itemMap[k.category] = [];
            }
            itemMap[k.category].push(k);
        }
        return itemMap;
    }, [items]);
    return (
        <motion.div
            className="w-1/3 max-w-[min(90vw,350px)] flex flex-col justify-start items-start gap-y-0 h-screen bg-fd-card px-3 py-4"
            animate={sidebar.open ? "open" : "close"}
            initial={sidebar.open ? "open" : "close"}
            variants={{
                open: {
                    translateX: 0,
                    width: "auto",
                },
                close: {
                    translateX: "-100%",
                    width: 0,
                },
            }}
        >
            <BlogSidebarTitlebar banner={banner} />
            {Object.keys(itemMap).map((item) => {
                return (
                    <BlogSidebarCategory
                        key={item}
                        category={item}
                        items={itemMap[item]}
                    />
                );
            })}
        </motion.div>
    );
};

BlogMainSidebar.displayName = "BlogMainSidebar";
