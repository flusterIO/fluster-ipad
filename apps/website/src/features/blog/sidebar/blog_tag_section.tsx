"use client";
import React, { useState, type ReactNode } from "react";
import { BlogSidebarSectionHeader } from "./blog_sidebar_section_header";
import { TagsIcon } from "lucide-react";
import { motion } from "framer-motion";
import { tagsToAllOfUrl } from "@conundrum/ts/pathUtils";

interface BlogTagSectionProps {
    tags: string[];
}

const SidebarTag = ({ tag, idx }: { tag: string; idx: number }): ReactNode => {
    return (
        <motion.a
            className="bg-primary text-sm px-2 py-1 rounded-full"
            href={tagsToAllOfUrl([tag], "/blog")}
            initial="close"
            animate={"open"}
            variants={{
                open: {
                    scale: 1,
                    opacity: 1,
                },
                close: {
                    scale: 0,
                    opacity: 0,
                },
            }}
            transition={{
                delay: 0.5 + 0.1 * idx,
            }}
        >
            {tag}
        </motion.a>
    );
};

export const BlogTagSection = ({ tags }: BlogTagSectionProps): ReactNode => {
    const [open, setOpen] = useState(true);
    return (
        <div className="w-full flex flex-col justify-start items-start">
            <BlogSidebarSectionHeader
                label="Tags"
                open={open}
                setOpen={setOpen}
                icon={<TagsIcon className="w-3 h-3 inline" />}
            />
            <motion.div
                className="w-full overflow-hidden flex flex-row flex-wrap  justify-start items-start gap-2 px-2 mt-2"
                animate={open ? "open" : "close"}
                initial={open ? "open" : "close"}
                variants={{
                    open: {
                        height: "fit-content",
                    },
                    close: {
                        height: 0,
                    },
                }}
            >
                {tags.length && open ? (
                    tags.map((t, i) => {
                        return <SidebarTag tag={t} idx={i} key={`${t}-${i}`} />;
                    })
                ) : open ? (
                    <div className="text-sm w-full text-center px-2 text-muted-foreground">
                        No tags found
                    </div>
                ) : null}
            </motion.div>
        </div>
    );
};

BlogTagSection.displayName = "BlogTagSection";
