import React, { type ReactNode } from "react";
import {
    PermanentSidebarButton,
    type PermanentSidebarButtonProps,
} from "./sidebar_button";
import {
    HomeIcon,
    SettingsIcon,
    BoxesIcon,
    FileSpreadsheet,
    WalletCards,
    HeartPulseIcon,
} from "lucide-react";
import { AppPaths } from "../app_paths";
import { useLocation } from "react-router";

const buttons: Omit<PermanentSidebarButtonProps, "active">[] = [
    {
        href: AppPaths.dashboard,
        icon: HomeIcon,
    },
    {
        href: AppPaths.workspaces,
        icon: BoxesIcon,
    },
    {
        href: AppPaths.flashcards,
        icon: WalletCards,
    },
    {
        href: AppPaths.database,
        icon: FileSpreadsheet,
    },
    {
        href: AppPaths.health,
        icon: HeartPulseIcon,
    },
];

export const PermanentSidebar = (): ReactNode => {
    const location = useLocation();
    return (
        <div className="left-0 top-0 bottom-0 h-screen w-16 bg-background border-r flex flex-col justify-between items-center py-6 gap-y-4">
            <div className="flex flex-col justify-start items-center gap-y-4">
                {buttons.map((b) => {
                    return (
                        <PermanentSidebarButton
                            active={
                                b.href
                                    ? b.href === "/"
                                        ? location.pathname === "/"
                                        : location.pathname.startsWith(b.href)
                                    : false
                            }
                            key={b.href ?? b.id}
                            {...b}
                        />
                    );
                })}
            </div>
            <PermanentSidebarButton
                active={location.pathname.startsWith(AppPaths.settings)}
                icon={SettingsIcon}
                href={AppPaths.settings}
            />
        </div>
    );
};

PermanentSidebar.displayName = "PermanentSidebar";
