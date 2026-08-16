import React, { type ReactNode, type ComponentProps } from "react";
import { Streamdown } from "streamdown";
import { code } from "@streamdown/code";
import { mermaid } from "@streamdown/mermaid";
import { math } from "@streamdown/math";
import { cn } from "@/utils/shad_utils";

interface StreamingMarkdownProps extends ComponentProps<typeof Streamdown> {
    children: string;
    activelyStreaming: boolean;
}

export const StreamingMarkdown = ({
    children,
    activelyStreaming,
    className,
}: StreamingMarkdownProps): ReactNode => {
    return (
        <Streamdown
            animated={true}
            isAnimating={activelyStreaming}
            skipHtml
            shikiTheme={["dracula", "dracula"]}
            plugins={{ code, mermaid, math }}
            linkSafety={{
                enabled: false,
            }}
            className={cn("*:text-foreground", className)}
        >
            {children}
        </Streamdown>
    );
};

StreamingMarkdown.displayName = "StreamingMarkdown";
