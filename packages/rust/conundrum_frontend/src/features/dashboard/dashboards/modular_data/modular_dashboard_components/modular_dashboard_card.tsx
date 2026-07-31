import { cn } from "@/utils/shad_utils";
import React, { type ReactNode } from "react";

interface ModularDashboardCardProps {
    children: ReactNode;
    title?: ReactNode;
    desc?: ReactNode;
    center?: boolean;
    className?: string;
}

export const ModularDashboardCard = ({
    title,
    desc,
    children,
    center,
    className,
}: ModularDashboardCardProps): ReactNode => {
    return (
        <div
            className={cn(
                "border rounded-xl bg-fd-card w-full h-full p-4",
                className,
            )}
        >
            {title ? (
                <>
                    <h5 className="text-sm font-semibold text-foreground/80">{title}</h5>
                    {desc ? <div>{desc}</div> : null}
                </>
            ) : null}
            {center ? <div className="w-full h-full flex flex-col justify-center items-center p-4">{children}</div> : null}
        </div>
    );
};

ModularDashboardCard.displayName = "ModularDashboardCard";
