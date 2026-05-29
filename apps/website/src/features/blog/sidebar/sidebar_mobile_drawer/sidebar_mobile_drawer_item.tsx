import React, { type ReactNode } from "react";
import { type BlogSidebarItem } from "../sidebar_category";

interface SidebarMobileDrawerItemProps {
    item: BlogSidebarItem;
}

export const SidebarMobileDrawerItem = ({
    item,
}: SidebarMobileDrawerItemProps): ReactNode => {
    return <a href={item.href} className="w-full block p-3 h-fit text-center">{item.label}</a>;
};

SidebarMobileDrawerItem.displayName = "SidebarMobileDrawerItem";
