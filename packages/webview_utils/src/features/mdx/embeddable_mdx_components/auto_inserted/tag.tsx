import { type TagResult } from "@/code_gen/typeshare/conundrum";
import { MdxPreviewWebviewActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { sendToSwift } from "@/utils/bridge/send_to_swift";
import React, { type ReactNode } from "react";

interface AutoInsertedTagProps {
    data: TagResult
}

export const AutoInsertedTag = ({
    data,
}: AutoInsertedTagProps): ReactNode => {
    return (
        <span
            role="button"
            className="bg-primary text-primary-foreground px-1 rounded-md cursor-pointer"
            onClick={() => {
                sendToSwift(MdxPreviewWebviewActions.OnTagClick, data.body);
            }}
        >
            {`#${data.body}`}
        </span>
    );
};

AutoInsertedTag.displayName = "AutoInsertedTag";
