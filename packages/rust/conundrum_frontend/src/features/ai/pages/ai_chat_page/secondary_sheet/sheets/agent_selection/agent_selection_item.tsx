import { type AgentDescription } from "#/database/db_utility_types/agent";
import React, { type ReactNode } from "react";

interface AgentSelectionItemProps {
    agent: AgentDescription;
    active: boolean;
}

export const AgentSelectionItem = (
    props: AgentSelectionItemProps,
): ReactNode => {
    return <div></div>;
};

AgentSelectionItem.displayName = "AgentSelectionItem";
