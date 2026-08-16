import React, { type ReactNode } from "react";

interface EmptyPanelProps {
    title: ReactNode;
    desc?: ReactNode;
}

export const EmptyPanel = ({ title, desc }: EmptyPanelProps): ReactNode => {
    return (
        <div className="w-fit h-fit flex flex-col justify-center items-center gap-y-2">
            <h5 className="text-lg font-semibold">{title}</h5>
            {desc ? (
                <div className="text-sm text-foreground/60 text-center max-w-[min(80%,350px)]">
                    These logs will be automatically generated as you use the Conundrum
                    ecosystem
                </div>
            ) : null}
        </div>
    );
};

EmptyPanel.displayName = "EmptyPanel";
