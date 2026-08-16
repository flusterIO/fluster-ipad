import { type EcosystemLogIntention } from "@/codegen/bindings";
import {
    ArrowUpRight,
    GitGraphIcon,
    HistoryIcon,
    PlusIcon,
    TrashIcon,
} from "lucide-react";
import React, { type FC, type ReactNode } from "react";

interface LogIntentionIconProps {
    intention: EcosystemLogIntention;
    className?: string;
}

export const LogIntentionIcon = ({
    intention,
    className,
}: LogIntentionIconProps): ReactNode => {
    const comps: Record<EcosystemLogIntention, FC<{ className?: string }>> = {
        "entity-created": PlusIcon,
        "git-status-change": GitGraphIcon,
        "entity-updated": ArrowUpRight,
        "entity-deleted": TrashIcon,
        "process-complete": HistoryIcon,
    };

    const Comp = comps[intention];

    return <Comp className={className} />;
};

LogIntentionIcon.displayName = "LogIntentionIcon";
