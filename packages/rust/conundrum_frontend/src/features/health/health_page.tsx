import { rspc } from "@/app/rspc_client";
import { PageContainer } from "@/components/general/page_container";
import React, { useMemo, type ReactNode } from "react";
import { DatabaseTableHealthItem } from "./database_table_health/database_table_health_item";
import { CenteredExpandedLoadingIndicator } from "#/navigation/full_screen_loading";
import { AnimatedCheckbox } from "#/onboarding/onboarding_screen/onboarding_checklist/onboarding_step";
import { PlainInlineCode } from "#/ui/typography/inline_code";
import { REMOTE_AI_ENVIRONMENT_VARIABLE } from "@/ai_constants";
import { cn } from "@/utils/shad_utils";

export const HealthPage = (): ReactNode => {
    const { data: health, isLoading } = rspc.useQuery(["rpc_health", null]);
    const { data: agentStatus } = rspc.useQuery(["backend_status", null]);
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
        <PageContainer>
            {isLoading ? (
                <CenteredExpandedLoadingIndicator />
            ) : health ? (
                <div className="w-full h-fit flex flex-col justify-start items-start">
                    <h2 className="text-2xl lg:text-3xl font-bold mb-3 grid grid-cols-[auto_32px]">
                        Connectivity
                        <div
                            className={cn(
                                "w-3 h-3 rounded-full place-self-center",
                                agentStatus?.is_online
                                    ? "animate-pulse bg-primary"
                                    : "bg-destructive animate-pulse",
                            )}
                        />
                    </h2>
                    <div className="bg-fd-card/50 border rounded p-3 w-full grid grid-cols-[auto_1fr] gap-x-3">
                        <AnimatedCheckbox
                            status={agentStatus?.local_client_access ? "complete" : "error"}
                            className="place-self-center"
                        />
                        <div className="flex flex-col justify-center items-start">
                            <h3 className="text-lg font-semibold">Local Inference</h3>
                            {agentStatus?.local_client_access ? (
                                <div>
                                    Your local environment is valid and local AI is available via
                                    Ollama.
                                </div>
                            ) : (
                                <div>
                                    There appears to be a problem with your Ollama environment. If
                                    you would like an offline fall-back, please setup a valid
                                    Ollama environment.
                                </div>
                            )}
                        </div>
                    </div>
                    <div className="bg-fd-card/50 border rounded p-3 w-full grid grid-cols-[auto_1fr] gap-x-3 mt-4">
                        <AnimatedCheckbox
                            status={agentStatus?.remote_client_access ? "complete" : "error"}
                            className="place-self-center"
                        />
                        <div className="flex flex-col justify-center items-start">
                            <h3 className="text-lg font-semibold">Remote Inference</h3>
                            {agentStatus?.remote_client_access ? (
                                <div>
                                    Your local environment is valid and remote AI is available for
                                    server scale tasks.
                                </div>
                            ) : agentStatus.is_online ? (
                                <div>
                                    There appears to be a problem with your{" "}
                                    <PlainInlineCode
                                        className="text-sm"
                                        color="code"
                                        code={REMOTE_AI_ENVIRONMENT_VARIABLE}
                                    />{" "}
                                    environment variable or another environment issue prohibiting
                                    Conundrum from connecting.
                                </div>
                            ) : (
                                <div>
                                    You appear to be offline so we can't connect to the remote
                                    server.{" "}
                                    {agentStatus?.local_client_access
                                        ? "You do however have access to a completely local offline fallback powered by Ollama."
                                        : "Unfortunately you don't have access to Ollama either, but you can still take advantage of the non-AI related feature of Conundrum."}
                                </div>
                            )}
                        </div>
                    </div>
                    <h3
                        className="text-2xl lg:text-3xl font-bold text-foreground my-5 flex flex-row justify-start items-center gap-x-2"
                        style={{
                            verticalAlign: "center",
                        }}
                    >
                        Table Health{" "}
                        <span className="text-lg lg:text-xl">
                            ({`${Math.round(percentSuccess)}%`})
                        </span>
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
                </div>
            ) : (
                <div className="w-full h-full flex flex-col justify-center items-center">
                    <div className="text-foreground text-lg font-bold">
                        Failed to load health
                    </div>
                </div>
            )}
        </PageContainer>
    );
};

HealthPage.displayName = "HealthPage";
