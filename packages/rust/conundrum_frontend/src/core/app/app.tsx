import { MainAppRouter } from "#/router/main_router";
import ReduxProvider from "@/state/redux_provider";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import React, { type ReactNode } from "react";
import { client, rspc } from "./rspc_client";

const queryClient = new QueryClient();

export const App = (): ReactNode => {
    return (
        <ReduxProvider>
            <rspc.Provider client={client} queryClient={queryClient}>
                <QueryClientProvider client={queryClient}>
                    <MainAppRouter />
                </QueryClientProvider>
            </rspc.Provider>
        </ReduxProvider>
    );
};

App.displayName = "App";
