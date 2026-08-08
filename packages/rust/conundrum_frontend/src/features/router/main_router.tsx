import React, { type ReactNode } from "react";
import { BrowserRouter } from "react-router";
import { MainAppRoutes } from "./routes";
import { PermanentSidebar } from "#/navigation/sidebar/permanent_sidebar";

export const MainAppRouter = (): ReactNode => {
    return (
        <BrowserRouter basename="/">
            <div className="app-container w-full h-screen max-h-screen flex flex-row justify-center items-center">
                <PermanentSidebar />
                <div className="@container/main bg-background w-[calc(100%-4rem)] max-h-screen min-h-screen overflow-x-hidden overflow-y-auto no-scrollbar">
                    <MainAppRoutes />
                </div>
            </div>
        </BrowserRouter>
    );
};

MainAppRouter.displayName = "MainAppRouter";
