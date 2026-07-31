import { PermanentSidebar } from "#/navigation/sidebar/permanent_sidebar";
import { MainAppRouter } from "#/router/main_router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import React, { type ReactNode } from "react";

/* const queryClient = new QueryClient(); */

export const App = (): ReactNode => {
    return (
        <div className="app-container w-full h-screen max-h-screen grid grid-cols-[auto_1fr]">
            <PermanentSidebar />
            <div className="@container/main w-full h-fit min-h-screen overflow-x-hidden overflow-y-auto min-scrollbar">
                <MainAppRouter />
            </div>
        </div>
    );
};

App.displayName = "App";
