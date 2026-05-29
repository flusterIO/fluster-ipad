"use client"
import React, { useEffect, useState, type ReactNode } from "react";
import { BlogSidebarCategory } from "./sidebar_category";
import { subjectToUrl, topicToUrl } from "@conundrum/ts/pathUtils";
import { BLOGURL } from "../constants";
import { LucideFolderPen, LucidePaperclip } from "lucide-react";
import { BlogTagSection } from "./blog_tag_section";
import { isMobile } from "react-device-detect";
import { SidebarMobileDrawer } from "./sidebar_mobile_drawer/sidebar_mobile_drawer";
import { MobileSidebarToggleButton } from "./sidebar_mobile_drawer/sidebar_mobile_toggle_button";

export interface BlogSidebarProps {
    topics: string[];
    subjects: string[];
    tags: string[];
    banner?: ReactNode;
}

export const BlogSidebar = (props: BlogSidebarProps): ReactNode => {
    const [preferMobileNav, setPreferMobileNav] = useState<null | boolean>(null);
    const handleIsMobileNav = (): void => {
        setPreferMobileNav(window.innerWidth < 768 || isMobile)
    }
    useEffect(() => {
        handleIsMobileNav()
        window.addEventListener("resize", handleIsMobileNav);
    }, [])
    if (preferMobileNav === null) {
        return null
    }
    if (preferMobileNav) {
        return (
            <>
                <SidebarMobileDrawer {...props} />
                <MobileSidebarToggleButton />
            </>
        )
    }
    return (
        <div className="max-w-[min(250px,90vw)] w-fit min-w-[150px] border-r h-screen  py-4">
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
            <BlogTagSection tags={props.tags} />
        </div>
    );
};

BlogSidebar.displayName = "BlogSidebar";
