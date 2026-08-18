import { rspc } from "@/app/rspc_client";
import { type BackendStatus } from "./db_utility_types/health";
import { AppPaths } from "#/navigation/app_paths";
import consola from "consola";
import { useEffect } from "react";
import { useLocation, useNavigate } from "react-router";

export const useBackendPinger = (): BackendStatus | null => {
    const { data } = rspc.useQuery(["backend_status", null], {
        retryOnMount: true,
        refetchOnWindowFocus: true,
        refetchInterval: 5 * 60 * 1000,
        refetchOnReconnect: "always",
    });
    const location = useLocation();
    const navigate = useNavigate();
    useEffect(() => {
        if (
            data?.all_tables_exist === false &&
            !location.pathname.startsWith(AppPaths.onboarding)
        ) {
            navigate(AppPaths.onboarding).catch((err: unknown) => {
                consola.error("Error: ", err);
            });
        }
    }, [data, location]);
    return data ?? null;
};
