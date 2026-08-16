import { humanReadableDateTime } from "@/utils/datetime_utils";
import React, { type ReactNode } from "react";

interface DateTimeComponentProps {
    dateTime: number | string;
    format: Parameters<typeof humanReadableDateTime>[1];
    className?: string;
    asSpan?: boolean;
}

export const DateTimeComponent = ({
    dateTime,
    format,
    className,
    asSpan,
}: DateTimeComponentProps): ReactNode => {
    if (asSpan) {
        return (
            <span className={className}>
                {humanReadableDateTime(new Date(dateTime), format)}
            </span>
        );
    }
    return (
        <div className={className}>
            {humanReadableDateTime(new Date(dateTime), format)}
        </div>
    );
};

DateTimeComponent.displayName = "DateTimeComponent";
