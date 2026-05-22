import React, { type ReactNode } from "react";
import consola from "consola";
import { type FileSummary } from "../types";
import { relativePathToBlogUrl } from "../../../path__utils/relative_path_to_url";

interface SidebarPanelItemProps {
    item: FileSummary;
}

export const SidebarPanelItem = ({
    item,
}: SidebarPanelItemProps): ReactNode => {
    if (!item.front_matter?.title) {
        consola.error(`Found a note without a title! ${item.relative_path}`);
        return;
    }
    if (!item.front_matter.summary) {
        consola.error(
            "Found a blog entry without a summary. You can give the user a better experience by providing a short summary. Here's the path: ",
            item.relative_path,
        );
        return;
    }
    if (!item.front_matter.user_defined_id) {
        consola.error(
            "Found a blog entry without a user defined id. We can't link to this entry by id anymore.",
            item.relative_path,
        );
        return;
    }
    return (
        <div className="w-full h-fit text-sm flex flex-col justify-start items-start border-b pl-2 pb-2">
            <a
                href={relativePathToBlogUrl(item.relative_path)}
                className="font-semibold"
            >
                {item.front_matter.title}
            </a>
            <div>{item.front_matter.summary}</div>
        </div>
    );
};

SidebarPanelItem.displayName = "SidebarPanelItem";
