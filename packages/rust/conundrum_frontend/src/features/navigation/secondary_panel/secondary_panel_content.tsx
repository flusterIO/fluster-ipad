import { cn } from "@/utils/shad_utils";
import React, { type ReactNode } from "react";

interface SecondaryPanelContentProps {
    title: ReactNode;
    desc?: ReactNode;
    children: ReactNode;
    classes?: {
        container?: string;
        childrenContainer?: string;
    };
    centerChildren?: boolean;
}

export const SecondaryPanelContent = ({
    title,
    desc,
    classes = {},
    children,
    centerChildren,
}: SecondaryPanelContentProps): ReactNode => {
    return (
        <div
            className={cn(
                "w-full h-fit min-h-full flex flex-col justify-start items-start",
                classes.container,
            )}
        >
            <h3 className="text-xl font-semibold">{title}</h3>
            {desc ? <div className="text-foreground/60 text-sm">{desc}</div> : null}
            <div
                className={cn(
                    "mt-3 grow",
                    centerChildren && "flex flex-col justify-center items-center w-full",
                    classes.childrenContainer,
                )}
            >
                {children}
            </div>
        </div>
    );
};

SecondaryPanelContent.displayName = "SecondaryPanelContent";
