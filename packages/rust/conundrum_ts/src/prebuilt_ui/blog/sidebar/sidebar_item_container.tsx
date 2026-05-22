"use client";
import React, { useEffect, useState, type ReactNode } from "react";
import { motion } from "framer-motion";

interface EventProps {
    open: "toggle" | boolean;
}

declare global {
    interface WindowEventMap {
        "toggle-blog-sidebar": CustomEvent<EventProps>;
    }
}

interface SidebarItemContainerProps {
    children: ReactNode;
}

export const SidebarItemContainer = ({
    children,
}: SidebarItemContainerProps): ReactNode => {
    const [open, setOpen] = useState(true);
    useEffect(() => {
        const handleEvent = (
            e: CustomEvent<{ open: boolean | "toggle" }>,
        ): void => {
            setOpen(e.detail.open === "toggle" ? !open : e.detail.open);
        };
        window.addEventListener("toggle-blog-sidebar", (e) => {
            handleEvent(e);
        });
        return () => {
            window.removeEventListener("toggle-blog-sidebar", handleEvent);
        };
    }, [open]);
    return (
        <motion.div
            className="sidebar-item-panel w-[min(200px,90vw)] max-w-[200px] h-screen left-[34px] top-0 bottom-0 fixed py-3 border-r bg-fd-card/60 overflow-hidden"
            animate={open ? "open" : "close"}
            initial={open ? "open" : "close"}
            variants={{
                open: {
                    translateX: 0,
                    z: 0,
                },
                close: {
                    translateX: "-100%",
                    z: -1,
                },
            }}
        >
            {children}
        </motion.div>
    );
};

SidebarItemContainer.displayName = "SidebarItemContainer";
