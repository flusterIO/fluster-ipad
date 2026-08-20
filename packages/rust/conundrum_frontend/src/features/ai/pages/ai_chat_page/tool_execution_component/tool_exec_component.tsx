import { type ToolExecution } from "@/codegen/bindings";
import React, { type ReactNode } from "react";

interface ToolExecComponentProps {
    item: ToolExecution;
}

export const ToolExecComponent = ({
    item,
}: ToolExecComponentProps): ReactNode => {
    return (
        <div className="w-full h-fit flex flex-row justify-center items-center">
            <div>{item.tool_name}</div>
        </div>
    );
};

ToolExecComponent.displayName = "ToolExecComponent";
