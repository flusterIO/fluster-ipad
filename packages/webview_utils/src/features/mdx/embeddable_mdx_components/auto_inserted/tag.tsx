import { sendToSwift, SwiftHandler } from "@/utils/bridge/send_to_swift";
import React, { type ReactNode } from "react";

interface AutoInsertedTagProps {
    children: string;
}

export const AutoInsertedTag = ({
    children,
}: AutoInsertedTagProps): ReactNode => {
    return (
        <span
            role="button"
            className="bg-primary text-primary-foreground px-1 rounded-md cursor-pointer"
            onClick={() => {
                sendToSwift(SwiftHandler.handleTagClick, children);
            }}
        >
            {`#${children}`}
        </span>
    );
};

AutoInsertedTag.displayName = "AutoInsertedTag";
