import React, { type ReactNode } from "react";

interface PathListItemProps {
    path: string;
}

export const PathListItem = ({ path }: PathListItemProps): ReactNode => {
    return <div>{path}</div>;
};

PathListItem.displayName = "PathListItem";
