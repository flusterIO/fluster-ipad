import { rspc } from "@/app/rspc_client";
import { PageContainer } from "@/components/general/page_container";
import React, { type ReactNode } from "react";
import { DatabaseTableHealthItem } from "./database_table_health/database_table_health_item";
import { CenteredExpandedLoadingIndicator } from "#/navigation/full_screen_loading";

export const HealthPage = (): ReactNode => {
    const { data: health, isLoading } = rspc.useQuery(["rpc_health", null]);
    return (
        <PageContainer title="Conundrum System Health">
            {isLoading ? (
                <CenteredExpandedLoadingIndicator />
            ) : health ? (
                <>
                    <h3 className="text-xl font-bold text-foreground my-5">
                        Table Health
                    </h3>
                    {health.table_reports.map((t) => {
                        return (
                            <DatabaseTableHealthItem item={t} key={t.description.table} />
                        );
                    })}
                </>
            ) : (
                <div>
                    <div className="text-foreground">Failed to load health</div>
                </div>
            )}
        </PageContainer>
    );
};

HealthPage.displayName = "HealthPage";
