import React, { useEffect, type ReactNode } from "react";
import { ModularDashboardCard } from "../../modular_dashboard_card";
import { rspc } from "@/app/rspc_client";
import { LoadingIndicator } from "#/navigation/loading_indicator";

export const AppVersionDataCard = (): ReactNode => {
    const res = rspc.useQuery(["version"]);
    useEffect(() => {
        console.log("res: ", res);
    }, [res]);
    return (
        <ModularDashboardCard title="Schema Version" center>
            {res.isPending ? <LoadingIndicator /> : res.data?.server}
        </ModularDashboardCard>
    );
};

AppVersionDataCard.displayName = "AppVersionDataCard";
