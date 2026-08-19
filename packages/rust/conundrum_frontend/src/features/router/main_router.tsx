import React, { useEffect, type ReactNode } from "react";
import { BrowserRouter, useLocation, useNavigate } from "react-router";
import { MainAppRoutes } from "./routes";
import { PermanentSidebar } from "#/navigation/sidebar/permanent_sidebar";
import { cn } from "@/utils/shad_utils";
import { AppPaths } from "#/navigation/app_paths";
import { SecondaryPanel } from "#/navigation/secondary_panel/secondary_panel";
import { NotificationsList } from "#/notifications/components/notification_list";
import { useGlobalKeyboardListener } from "#/keyboard/use_global_keyboard_listener";
import { CommandPalette } from "#/command_palette/command_palette";
import { CommandPaletteProvider } from "#/command_palette/command_palette_provider";
import { GlobalKeyboardListener } from "#/command_palette/global_keyboard_listener";
import { useBackendPinger } from "#/database/backend_pinger";

const dontScroll: AppPaths[] = [
    AppPaths.onboarding
]

const MainRoutes = (): ReactNode => {
    const location = useLocation();
    useGlobalKeyboardListener();
    useBackendPinger();
    return (
        <div
            className={cn(
                "@container/main bg-background w-[calc(100%-4rem)] max-h-screen min-h-screen overflow-x-hidden no-scrollbar",
                !dontScroll.some((x) => {
                    return location.pathname.startsWith(x)
                }) && "overflow-y-auto ",
                location.pathname.startsWith(AppPaths.onboarding) && "w-screen",
            )}
        >
            <MainAppRoutes />
        </div>
    );
};

export const MainAppRouter = (): ReactNode => {
    return (
        <BrowserRouter basename="/">
            <div className="app-container w-full h-screen max-h-screen flex flex-row justify-center items-center">
                <PermanentSidebar />
                <MainRoutes />
                <CommandPaletteProvider>
                    <GlobalKeyboardListener />
                    <CommandPalette />
                </CommandPaletteProvider>
                <NotificationsList />
                <SecondaryPanel />
            </div>
        </BrowserRouter>
    );
};

MainAppRouter.displayName = "MainAppRouter";
