import { cn } from "@/utils/shad_utils";
import React, { type ReactNode } from "react";

interface SettingsSectionProps {
    label: ReactNode;
    desc?: ReactNode;
    children: ReactNode;
    className?: string;
}

export const SettingsSection = ({
    label,
    desc,
    children,
    className,
}: SettingsSectionProps): ReactNode => {
    return (
        <div className={cn("w-full max-w-270 mx-auto my-6", className)}>
            <div className="h-fit w-full">
                <h3
                    className={cn("text-3xl font-bold text-foreground", !desc && "mb-4")}
                >
                    {label}
                </h3>
                {desc ? (
                    <div className="text-sm text-muted-foreground mb-4">{desc}</div>
                ) : null}
            </div>
            {children}
        </div>
    );
};

SettingsSection.displayName = "SettingsSection";
