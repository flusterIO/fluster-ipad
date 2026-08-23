import React, { useMemo, type ReactNode } from "react";
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
    MessageSquare,
    PersonStanding,
} from "lucide-react";
import { AppPaths } from "../app_paths";
import { useLocation } from "react-router";
import { type AppState } from "@/state/initial_state";
import { useSelector } from "react-redux";
import { v4 } from "uuid";

export const PermanentSidebar = (): ReactNode => {
    const location = useLocation();
    const dailyChat = useSelector((state: AppState) => {
        return {
            dailyChat: state.ai.dailyChat,
            mostRecentChat: state.ai.mostRecentChat,
        };
    });
    if (
        location.pathname.startsWith(AppPaths.onboarding) ||
        !dailyChat.dailyChat
    ) {
        return null;
    }
    const buttons = useMemo(() => {
        const searchParams = new URLSearchParams();
        const chatId = dailyChat?.dailyChat?.was_directed
            ? (dailyChat.mostRecentChat ?? dailyChat.dailyChat.chat_id)
            : dailyChat.dailyChat?.chat_id;
        searchParams.set("convo", chatId ?? v4());
        const btns: Omit<PermanentSidebarButtonProps, "active">[] = [
            {
                href: AppPaths.dashboard,
                icon: HomeIcon,
            },
            {
                href: `${AppPaths.aiChat}?${searchParams.toString()}`,
                icon: MessageSquare,
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
                href: AppPaths.agents,
                icon: PersonStanding,
            },
            {
                href: AppPaths.health,
                icon: HeartPulseIcon,
            },
        ];
        return btns;
    }, [location.pathname]);
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
                                        : location.pathname.startsWith(b.href) ||
                                        (location.pathname.startsWith(AppPaths.aiChat) &&
                                            b.href.startsWith(location.pathname))
                                    : false
                            }
                            key={b.href}
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
