import { type EcosystemLogItem } from "#/database/db_utility_types/log_types";
import React, { type ReactNode } from "react";
import { LogIntentionIcon } from "./log_intention_icon";
import { type EcosystemLogSeverity } from "@/codegen/bindings";
import { DateTimeComponent } from "#/datetime/components/date_time";

interface LogItemComponentProps {
    item: EcosystemLogItem;
}

export const logSeverityToColorClasses = (
    severity: EcosystemLogSeverity,
): string => {
    switch (severity) {
        case "information":
            return "bg-blue-500 text-white";
        case "success":
            return "bg-green-500 text-foreground";
        case "warning":
            return "bg-yellow-500 text-black";
        case "error":
            return "bg-destructive text-destructive-foreground";
    }
};

export const LogItemComponent = ({
    item,
}: LogItemComponentProps): ReactNode => {
    return (
        <div className="w-full flex flex-col jutify-between items-start rounded p-4 border bg-fd-card text-fd-card-foreground">
            <div className="w-full flex flex-row justify-start items-center gap-x-2">
                <LogIntentionIcon intention={item.purpose} />
                <h5 className="text-lg font-semibold">{item.title}</h5>
            </div>
            {item.message ? (
                <div className="w-full h-fit text-foreground/80">{item.message}</div>
            ) : null}
            <DateTimeComponent
                className="text-sm text-foreground/60 mt-2"
                dateTime={item.ctime}
                format="full-with-time"
            />
        </div>
    );
};

LogItemComponent.displayName = "LogItemComponent";
