"use client";
import { ChevronDown as CD } from "lucide-react";
import React, { type ReactNode } from "react";
import { motion } from "framer-motion";
const ChevronDown = motion.create(CD);

interface BlogSidebarSectionHeaderProps {
    label: string;
    icon: ReactNode;
    open: boolean;
    setOpen: (newOpen: boolean) => void;
}

export const BlogSidebarSectionHeader = ({
    label,
    icon,
    open,
    setOpen,
}: BlogSidebarSectionHeaderProps): ReactNode => {
    return (
        <div
            className="font-semibold w-full grid grid-cols-[auto_1fr_auto] place-items-center gap-x-2 px-4 mt-3"
            onClick={() => {
                setOpen(!open);
            }}
        >
            {icon}
            <div className="w-full text-left">{label}</div>
            <ChevronDown
                className="inline text-foreground/80"
                animate={open ? "open" : "close"}
                initial={open ? "open" : "close"}
                variants={{
                    open: {
                        rotateZ: 0,
                    },
                    close: {
                        rotateZ: 180,
                    },
                }}
            />
        </div>
    );
};

BlogSidebarSectionHeader.displayName = "BlogSidebarSectionHeader";
