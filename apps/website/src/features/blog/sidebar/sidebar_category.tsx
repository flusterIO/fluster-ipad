"use client";
import { cn } from "#/lib/cn";
import { useState, type ReactNode } from "react";
import { BlogSidebarSectionHeader } from "./blog_sidebar_section_header";
import { motion } from "framer-motion";

export interface BlogSidebarItem {
    label: string;
    href: string;
    active?: boolean;
}

export interface BlogSidebarCategoryProps {
    label: string;
    items: BlogSidebarItem[];
    icon: ReactNode;
}

export const BlogSidebarCategory = ({
    label,
    items,
    icon,
}: BlogSidebarCategoryProps): ReactNode => {
    const [open, setOpen] = useState(true);
    return (
        <div className="">
            <BlogSidebarSectionHeader
                label={label}
                open={open}
                setOpen={setOpen}
                icon={icon}
            />
            <motion.div
                className="overflow-hidden"
                animate={open ? "open" : "close"}
                initial={open ? "open" : "close"}
                variants={{
                    open: {
                        height: "fit-content",
                    },
                    close: {
                        height: "0px",
                    },
                }}
            >
                {items.length ? (
                    items.map((item) => {
                        return (
                            <a
                                key={`${item.href}-${item.label}`}
                                href={item.href}
                                className={cn(
                                    "text-muted-foreground text-sm pl-6 hover:text-foreground/90 transition-colors duration-300",
                                    item.active && "text-foreground/90",
                                )}
                            >
                                {item.label}
                            </a>
                        );
                    })
                ) : (
                    <div className="text-muted-foreground text-sm w-full text-center">
                        Nothing Found
                    </div>
                )}
            </motion.div>
        </div>
    );
};
