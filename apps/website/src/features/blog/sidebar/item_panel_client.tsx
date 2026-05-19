"use client";
import { SidebarItemContainer } from "@conundrum/ts/ui/blog";
import React, { type ReactNode } from "react";

export const SidebarItemPanelClient = ({
    children,
}: {
    children: ReactNode;
}): ReactNode => {
    return <SidebarItemContainer>{children}</SidebarItemContainer>;
};

SidebarItemPanelClient.displayName = "SidebarItemPanelClient";
