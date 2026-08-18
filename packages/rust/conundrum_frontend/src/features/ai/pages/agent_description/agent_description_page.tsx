import { PageContainer } from "@/components/general/page_container";
import React, { type ReactNode } from "react";
import { AgentForm } from "./agent_form/agent_form";
import { useSearchParams } from "react-router";
import { CenteredExpandedLoadingIndicator } from "#/navigation/full_screen_loading";

export const AgentDescriptionPage = (): ReactNode => {
    const [sp] = useSearchParams();
    const agentId = sp.get("agent");
    return (
        <PageContainer title="Agent">
            {agentId ? (
                <AgentForm agentId={agentId} />
            ) : (
                <CenteredExpandedLoadingIndicator />
            )}
        </PageContainer>
    );
};

AgentDescriptionPage.displayName = "AgentDescriptionPage";
