import React, { type ReactNode } from "react";
import { type BlogCategory } from "../types";

interface PermanentLeftSidebarProps {
    topics: BlogCategory[];
    toggleButton: ReactNode;
}

export const PermanentLeftSidebar = ({
    topics,
    toggleButton,
}: PermanentLeftSidebarProps): ReactNode => {
    return (
        <div className="h-screen flex flex-col justify-between items-center fixed top-0 left-0 bottom-0 w-[32px] border-r py-3 bg-card z-10">
            <div className="flex-grow flex flex-col justify-start items-center gap-y-2">
                {topics.map((item) => {
                    const Icon = item.icon;
                    return <Icon key={item.id ?? item.label} className="w-8 h-8 p-1" />;
                })}
            </div>
            <div>{toggleButton}</div>
        </div>
    );
};

PermanentLeftSidebar.displayName = "PermanentLeftSidebar";
