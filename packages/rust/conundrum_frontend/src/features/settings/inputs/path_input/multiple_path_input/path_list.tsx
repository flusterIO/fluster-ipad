import React, { type ReactNode } from "react";
import { PathListItem } from "./path_list_item";

interface PathListProps {
    paths: string[];
}

export const PathList = ({ paths }: PathListProps): ReactNode => {
    return (
        <div className="w-full flex flex-col justify-center items-center">
            {paths.map((p) => {
                return <PathListItem path={p} />;
            })}
        </div>
    );
};

PathList.displayName = "PathList";
