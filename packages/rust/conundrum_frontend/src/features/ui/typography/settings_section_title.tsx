import { cn } from "@/utils/shad_utils";
import React, { type ReactNode } from "react";

interface SettingsSectionTitleProps {
    children: ReactNode;
    desc?: ReactNode;
    classes?: {
        container?: string;
        title?: string;
        desc?: string;
    };
}

export const SettingsSectionTitle = ({
    children,
    desc,
    classes = {},
}: SettingsSectionTitleProps): ReactNode => {
    if (desc) {
        return (
            <div className={cn("w-full mt-6 mb-4", classes.container)}>
                <h3
                    className={cn(
                        "text-2xl font-semibold text-foreground",
                        classes.title,
                    )}
                >
                    {children}
                </h3>
                <div className={cn("text-foreground/60 text-sm", classes.desc)}>
                    {desc}
                </div>
            </div>
        );
    }
    return (
        <h3
            className={cn(
                "text-2xl font-semibold mt-6 mb-4 text-foreground",
                classes.title,
            )}
        >
            {children}
        </h3>
    );
};

SettingsSectionTitle.displayName = "SettingsSectionTitle";
