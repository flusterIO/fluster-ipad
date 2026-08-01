import { cn } from "@/utils/shad_utils";
import React, { type ReactNode } from "react";

interface SettingsFieldDescriptionProps {
    children: ReactNode;
    className?: string;
}

export const SettingsFieldDescription = ({
    children,
    className,
}: SettingsFieldDescriptionProps): ReactNode => {
    return (
        <div className={cn("text-sm text-muted-foreground mt-3", className)}>
            {children}
        </div>
    );
};

SettingsFieldDescription.displayName = "SettingsFieldDescription";
