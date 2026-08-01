import React, { type ReactNode } from "react";
import { BrowserRouter } from "react-router";
import { MainAppRoutes } from "./routes";
import { PermanentSidebar } from "#/navigation/sidebar/permanent_sidebar";

export const MainAppRouter = (): ReactNode => {
    return (
        <BrowserRouter basename="/">
            <div className="app-container w-full h-screen max-h-screen grid grid-cols-[auto_1fr]">
                <PermanentSidebar />
                <div className="@container/main w-full h-fit min-h-screen overflow-x-hidden overflow-y-auto min-scrollbar">
                    <MainAppRoutes />
                </div>
            </div>
        </BrowserRouter>
    );
};

MainAppRouter.displayName = "MainAppRouter";
