import React, { type ReactNode } from "react";
import { SidebarItemContainer } from "./sidebar_item_container";

interface SidebarItemPanelProps {
    children: ReactNode
}

export const SidebarItemPanel = ({
    children
}: SidebarItemPanelProps): ReactNode => {
    return (
        <SidebarItemContainer>
            {children}
        </SidebarItemContainer>
    );
};

SidebarItemPanel.displayName = "SidebarItemPanel";
