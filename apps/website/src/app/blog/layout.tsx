import type { ReactNode } from "react";
import "../../core/styles/docs.scss";
import data from "../../features/cdrm/cdrm.json";
import { BlogSidebar } from "#/features/blog/sidebar/blog_sidebar";
import { LockBodyScrollHack } from "./lock_body_scroll_hack";
import { SidebarBanner } from "#/features/blog/sidebar/banner";
import { BlogDevelopmentWarning } from "#/features/blog/under_development_warning/blog_development_warning";

const blacklist_taggables = ["LinkOnly"];

export default function Layout({ children }: { children: ReactNode }) {
    const subjects: string[] = [];
    const topics: string[] = [];
    const tags: string[] = [];
    for (const f of data.files) {
        const subject = f.front_matter?.subject;
        if (
            subject &&
            !subjects.includes(subject) &&
            !blacklist_taggables.includes(subject)
        ) {
            subjects.push(subject);
        }

        const topic = f.front_matter?.topic;
        if (
            topic &&
            !topics.includes(topic) &&
            !blacklist_taggables.includes(topic)
        ) {
            topics.push(topic);
        }

        for (const t of f.tags) {
            if (!tags.includes(t) && !blacklist_taggables.includes(t)) {
                tags.push(t);
            }
        }
    }
    return (
        <div
            data-cdrm-sidebar="open"
            className="w-full h-fit flex flex-col justify-center items-center h-scren max-h-screen overflow-hidden grid grid-cols-[auto_1fr]"
        >
            <LockBodyScrollHack />
            <BlogDevelopmentWarning />
            <BlogSidebar
                tags={tags}
                subjects={subjects}
                topics={topics}
                banner={<SidebarBanner />}
            />
            {children}
        </div>
    );
}
