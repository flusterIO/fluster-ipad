import type { ReactNode } from "react";
import "../../core/styles/docs.scss";
import data from "../../features/cdrm/cdrm.json";
import { BlogSidebar } from "#/features/blog/sidebar/blog_sidebar";

export default function Layout({ children }: { children: ReactNode }) {
    const subjects: string[] = [];
    const topics: string[] = [];
    const tags: string[] = [];
    for (const f of data.files) {
        const subject = f.front_matter?.subject as null | undefined | string;
        if (subject && !subjects.includes(subject)) {
            subjects.push(subject);
        }

        const topic = f.front_matter?.topic as null | undefined | string;
        if (topic && !topics.includes(topic)) {
            topics.push(topic);
        }

        for (const t of f.tags) {
            if (!tags.includes(t)) {
                tags.push(t);
            }
        }
    }
    return (
        <div
            data-cdrm-sidebar="open"
            className="w-full h-fit flex flex-col justify-center items-center"
        >
            <BlogSidebar tags={tags} subjects={subjects} topics={topics} />
            {children}
        </div>
    );
}
