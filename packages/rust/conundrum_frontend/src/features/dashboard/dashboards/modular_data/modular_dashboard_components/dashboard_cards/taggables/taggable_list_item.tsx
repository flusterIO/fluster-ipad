import { type AppPaths } from "#/navigation/app_paths";
import React, { type ReactNode } from "react";
import { Link } from "react-router";

interface TaggableListItemProps {
    /**
     * The link to the search page
     */
    href: `${AppPaths.search}?${string}`;
    children: ReactNode;
}

export const TaggableListItem = ({
    href,
    children,
}: TaggableListItemProps): ReactNode => {
    console.log("children: ", children);
    return (
        <Link
            to={href}
            className="w-full text-foreground/80! hover:text-foreground! px-4"
        >
            {children}
        </Link>
    );
};

TaggableListItem.displayName = "TaggableListItem";
