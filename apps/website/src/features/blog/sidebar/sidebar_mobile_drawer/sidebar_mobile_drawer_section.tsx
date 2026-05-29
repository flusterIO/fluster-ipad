import React, { type ReactNode } from "react";
import { type BlogSidebarCategoryProps } from "../sidebar_category";
import { SidebarMobileDrawerItem } from "./sidebar_mobile_drawer_item";

export const SidebarMobileDrawerSection = (
    props: BlogSidebarCategoryProps,
): ReactNode => {
    return (
        <div className="w-full h-fit">
            <div className="grid grid-cols-[1fr_auto_1fr] place-items-center my-3 gap-x-4 px-4">
                <div className="w-full h-[2px] bg-border" />
                <div className="text-sm text-muted-foreground">{props.label}</div>
                <div className="w-full h-[2px] bg-border" />
            </div>
            {props.items.map((item) => {
                return <SidebarMobileDrawerItem item={item} key={item.href} />;
            })}
        </div>
    );
};

SidebarMobileDrawerSection.displayName = "SidebarMobileDrawerSection";
