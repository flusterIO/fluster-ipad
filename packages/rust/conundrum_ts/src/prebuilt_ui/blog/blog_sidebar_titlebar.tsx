import { PanelLeftOpen as PL } from "lucide-react";
import React, { type ReactNode } from "react";
import { motion } from "framer-motion";
import {
    useBlogProviderContext,
    useBlogProviderDispatch,
} from "./state/blog_provider";
const PanelLeftOpen = motion.create(PL);

interface BlogSidebarTitlebarProps {
    banner?: ReactNode;
}

export const BlogSidebarTitlebar = ({
    banner = "Conundrum",
}: BlogSidebarTitlebarProps): ReactNode => {
    const { sidebar } = useBlogProviderContext();
    const dispatch = useBlogProviderDispatch();
    const setOpen = (val: boolean) => {
        dispatch({
            type: "toggleSidebar",
            payload: val,
        });
    };
    return (
        <div className="w-full flex flex-row justify-between items-center mb-3">
            <div className="font-semibold text-lg">{banner}</div>
            <PanelLeftOpen
                className="cursor-pointer text-foreground opacity-40 w-4 h-4"
                animate={sidebar.open ? "open" : "close"}
                variants={{
                    open: {
                        translateX: 0,
                    },
                    close: {
                        translateX: "120px",
                    },
                }}
                onClick={() => {
                    setOpen(!open);
                }}
            />
        </div>
    );
};

BlogSidebarTitlebar.displayName = "BlogSidebarTitlebar";
