import { rspc } from "@/app/rspc_client";
import { type BackendStatus } from "./db_utility_types/health";
import { AppPaths } from "#/navigation/app_paths";
import { useEffect } from "react";
import { type useLocation, useNavigate } from "react-router";

const useNavigateToOnboardingOnPingerFail = (
    data: BackendStatus | undefined,
    location: ReturnType<typeof useLocation>,
): void => {
    const navigate = useNavigate();

    useEffect(() => {
        if (
            data?.all_tables_exist === false &&
            !location.pathname.startsWith(AppPaths.onboarding)
        ) {
            // eslint-disable-next-line @typescript-eslint/no-floating-promises
            navigate(AppPaths.onboarding);
        }
    }, [data, location]);
};

export const useBackendPinger = (): BackendStatus | null => {
    const { data } = rspc.useQuery(["backend_status", null], {
        retryOnMount: true,
        refetchOnWindowFocus: true,
        refetchInterval: 5 * 60 * 1000,
        refetchOnReconnect: "always",
    });
    // const location = useLocation();
    // TODO: TURN THIS BACK ON FOR PRODUCTION.
    // useNavigateToOnboardingOnPingerFail(data, location);
    return data ?? null;
};
