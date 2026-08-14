import { rspc } from "@/app/rspc_client";
import { PageContainer } from "@/components/general/page_container";
import React, { useMemo, type ReactNode } from "react";
import { DatabaseTableHealthItem } from "./database_table_health/database_table_health_item";
import { CenteredExpandedLoadingIndicator } from "#/navigation/full_screen_loading";

export const HealthPage = (): ReactNode => {
    const { data: health, isLoading } = rspc.useQuery(["rpc_health", null]);
    const percentSuccess = useMemo(() => {
        if (!health) {
            return 0;
        }
        let total_valid = 0;
        let total_invalid = 0;
        for (const k of health.table_reports) {
            if (k.is_temporary_table || k.exists) {
                total_valid += 1;
            } else {
                total_invalid += 1;
            }
        }
        return (total_valid / (total_invalid + total_valid)) * 100;
    }, [health]);
    return (
        <PageContainer title="Conundrum System Health">
            {isLoading ? (
                <CenteredExpandedLoadingIndicator />
            ) : health ? (
                <>
                    <h3 className="text-xl font-bold text-foreground my-5">
                        {`Table Health (${Math.round(percentSuccess)}%)`}
                    </h3>
                    <div className="w-full flex flex-col justify-center items-center gap-y-4">
                        {health.table_reports
                            .sort((a, b) =>
                                b.description.entity_name >= a.description.entity_name ? -1 : 1,
                            )
                            .map((t) => {
                                return (
                                    <DatabaseTableHealthItem item={t} key={t.description.table} />
                                );
                            })}
                    </div>
                </>
            ) : (
                <div className="w-full h-full flex flex-col justify-center items-center">
                    <div className="text-foreground text-lg font-bold">Failed to load health</div>
                </div>
            )}
        </PageContainer>
    );
};

HealthPage.displayName = "HealthPage";
