import { cn } from "@/utils/shad_utils";
import React, { type ReactNode } from "react";

interface SettingsSectionProps {
    label: ReactNode;
    desc?: ReactNode;
    children: ReactNode;
}

export const SettingsSection = ({
    label,
    desc,
    children,
}: SettingsSectionProps): ReactNode => {
    return (
        <div className="w-full max-w-270 mx-auto my-6">
            <h3 className={cn("text-xl font-semibold", !desc && "mb-4")}>{label}</h3>
            {desc ? (
                <div className="text-sm text-muted-foreground mb-4">{desc}</div>
            ) : null}
            {children}
        </div>
    );
};

SettingsSection.displayName = "SettingsSection";
