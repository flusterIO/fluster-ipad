import { StreamingMarkdown } from "#/streaming/markdown/streaming_markdown";
import { type ReasoningBlock } from "@/codegen/bindings";
import React, { type ReactNode } from "react";

interface ReasoningTextComponentProps {
    item: ReasoningBlock;
}

export const ReasoningTextComponent = ({
    item,
}: ReasoningTextComponentProps): ReactNode => {
    return (
        <StreamingMarkdown activelyStreaming={false}>
            {item.content}
        </StreamingMarkdown>
    );
};

ReasoningTextComponent.displayName = "ReasoningTextComponent";
