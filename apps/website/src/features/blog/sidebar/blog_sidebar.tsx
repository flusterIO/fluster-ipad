import React, { type ReactNode } from "react";
import { BlogSidebarCategory } from "./sidebar_category";
import { subjectToUrl, topicToUrl } from "@conundrum/ts/pathUtils";

interface BlogSidebarProps {
    topics: string[];
    subjects: string[];
    tags: string[];
}

export const BlogSidebar = (props: BlogSidebarProps): ReactNode => {
    return (
        <div className="max-w-[min(350px,90vw)] w-[350px] border-r">
            <BlogSidebarCategory
                label="Topics"
                items={props.topics.map((t) => {
                    return {
                        label: t,
                        href: topicToUrl(t, BLOGURL),
                    };
                })}
            />
            <BlogSidebarCategory
                label="Subjects"
                items={props.subjects.map((t) => {
                    return {
                        label: t,
                        href: subjectToUrl(t, BLOGURL),
                    };
                })}
            />
        </div>
    );
};

BlogSidebar.displayName = "BlogSidebar";
