"use client";
import React, { type ReactNode, useState } from "react";
import { motion } from "framer-motion";
import { ChevronDown as CD } from "lucide-react";

const ChevronDown = motion.create(CD);

type X = { onClick: () => void } | { href: string };

export type BlogSidebarItem = {
    label: string;
    category: string;
    /// Any unique id. This is only required if there mght be a label that appears twice.
    id?: string;
} & X;

export const BlogSidebarCategory = ({
    items,
    category,
}: {
    items: BlogSidebarItem[];
    category: string;
}): ReactNode => {
    const [open, setOpen] = useState(false);
    return (
        <motion.div
            onClick={() => {
                setOpen(!open);
            }}
        >
            <div className="font-semibold cursor-pointer grid grid-cols-[auto_1fr] gap-x-1">
                <ChevronDown
                    className="w-3 h-3 place-self-center"
                    animate={open ? "open" : "close"}
                    initial={open ? "open" : "close"}
                    variants={{
                        open: {
                            rotateZ: 0,
                        },
                        close: {
                            rotateZ: -90,
                        },
                    }}
                />
                {category}
            </div>
            <motion.div
                className="pl-4 overflow-hidden"
                animate={open ? "open" : "close"}
                initial={open ? "open" : "close"}
                variants={{
                    open: {
                        height: "auto",
                    },
                    close: {
                        height: 0,
                    },
                }}
            >
                {items.map((item) => {
                    if ("href" in item) {
                        return <a href={item.href}>{item.label}</a>;
                    }
                    return (
                        <div onClick={"onClick" in item ? item.onClick : undefined}>
                            {item.label}
                        </div>
                    );
                })}
            </motion.div>
        </motion.div>
    );
};
