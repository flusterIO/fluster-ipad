import React, { type ReactNode } from "react";
import { BrowserRouter, useLocation } from "react-router";
import { MainAppRoutes } from "./routes";
import { PermanentSidebar } from "#/navigation/sidebar/permanent_sidebar";
import { cn } from "@/utils/shad_utils";
import { AppPaths } from "#/navigation/app_paths";

const MainRoutes = (): ReactNode => {
    const location = useLocation();
    return (
        <div
            className={cn(
                "@container/main bg-background w-[calc(100%-4rem)] max-h-screen min-h-screen overflow-x-hidden overflow-y-auto no-scrollbar",
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
            </div>
        </BrowserRouter>
    );
};

MainAppRouter.displayName = "MainAppRouter";
