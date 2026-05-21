import React, { type ReactNode } from "react";
import { BlogSidebarCategory } from "./sidebar_category";
import { subjectToUrl, topicToUrl } from "@conundrum/ts/pathUtils";
import { BLOGURL } from "../constants";
import { LucideFolderPen, LucidePaperclip } from "lucide-react";

interface BlogSidebarProps {
    topics: string[];
    subjects: string[];
    tags: string[];
    banner?: ReactNode;
}

export const BlogSidebar = (props: BlogSidebarProps): ReactNode => {
    return (
        <div className="max-w-[min(250px,90vw)] w-[250px] min-w-[250px] border-r h-screen  py-4">
            {props.banner ?? null}
            <BlogSidebarCategory
                label="Topics"
                icon={<LucideFolderPen className="w-3 h-3" />}
                items={props.topics.map((t) => {
                    return {
                        label: t,
                        href: topicToUrl(t, BLOGURL),
                    };
                })}
            />
            <BlogSidebarCategory
                label="Subjects"
                icon={<LucidePaperclip className="w-3 h-3" />}
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
