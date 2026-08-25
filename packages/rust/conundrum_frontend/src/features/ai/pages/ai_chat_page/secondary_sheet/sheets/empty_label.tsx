import React, { type ReactNode } from "react";

interface ChatPanelEmptyLabelProps {
    title: ReactNode;
    body: ReactNode;
}

export const ChatPanelEmptyLabel = ({
    title,
    body,
}: ChatPanelEmptyLabelProps): ReactNode => {
    return (
        <div className="w-full h-fit flex flex-col justify-center items-center grow">
            <h3 className="text-foreground font-semibold text-lg text-center">
                {title}
            </h3>
            <p className="text-foreground/80 font-sm text-center">{body}</p>
        </div>
    );
};

ChatPanelEmptyLabel.displayName = "ChatPanelEmptyLabel";
