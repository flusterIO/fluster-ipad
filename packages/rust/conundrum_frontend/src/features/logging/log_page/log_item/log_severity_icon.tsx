import { type EcosystemLogSeverity } from "@/codegen/bindings";
import { BugIcon, CheckIcon, InfoIcon, TriangleAlert } from "lucide-react";
import React, { type ReactNode, type FC } from "react";

interface LogSeverityIconProps {
    severity: EcosystemLogSeverity;
    className?: string;
}

export const LogSeverityIcon = ({
    className,
    severity,
}: LogSeverityIconProps): ReactNode => {
    const comps: Record<EcosystemLogSeverity, FC<{ className?: string }>> = {
        error: BugIcon,
        warning: TriangleAlert,
        information: InfoIcon,
        success: CheckIcon,
    };

    const Comp = comps[severity];

    return <Comp className={className} />;
};

LogSeverityIcon.displayName = "LogSeverityIcon";
