"use client";
import { type BlogTemplateProps } from "@conundrum/ts/ui/blog";
import dynamic from "next/dynamic";
import React, { type ReactNode } from "react";
import cdrmOutput from "../../../features/cdrm/cdrm.json";
const BlogTemplate = dynamic(
    () => import("@conundrum/ts/ui/blog").then((a) => a.BlogTemplate),
    { ssr: false },
);

export const BlogTemplateClientWrapper = ({
    searchParams,
    sidebarItems,
    banner,
}: Omit<BlogTemplateProps, "data">): ReactNode => {
    return (
        <BlogTemplate
            sidebarItems={sidebarItems}
            searchParams={searchParams}
            /* @ts-expect-error -- 'I'll deal witht his later. It's just a type issue with null and undefined.*/
            data={cdrmOutput}
            banner={banner}
        />
    );
};

BlogTemplateClientWrapper.displayName = "BlogTemplateClientWrapper";
