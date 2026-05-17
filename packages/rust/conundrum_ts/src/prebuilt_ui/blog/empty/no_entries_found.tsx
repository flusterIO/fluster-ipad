import React, { type ReactNode } from "react";

export const NoBlogEntriesFound = (): ReactNode => {
    return (
        <div className="w-full h-fit text-xl font-semibold text-center py-4 px-3">
            No entries found
        </div>
    );
};

NoBlogEntriesFound.displayName = "NoBlogEntriesFound";
