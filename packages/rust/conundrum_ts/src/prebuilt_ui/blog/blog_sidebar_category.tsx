"use client"
import React, { type ReactNode, useState } from "react";
import { motion } from "framer-motion";

type X = { onClick: string } | { href: string };

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
        <motion.div onClick={() => { setOpen(!open); }}>
            <div className="font-semibold">{category}</div>
            <motion.div className="pl-2">
                {items.map((item) => {
                    return <div>{item.label}</div>;
                })}
            </motion.div>
        </motion.div>
    );
};
