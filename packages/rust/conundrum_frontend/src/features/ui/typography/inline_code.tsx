import React, { type ReactNode } from "react";
import {
    useHighlightedCode,
    type HighlightCodeRequest,
} from "../state/hooks/use_highlighted_code";
import { cn } from "@/utils/shad_utils";

export const PlainInlineCode = ({
    code,
    background = false,
    color = "inherit",
}: {
    code: string;
    color?: "inherit" | "code";
    background?: boolean;
}) => {
    return (
        <span
            className={cn(
                "font-mono",
                color === "code" && "text-lime-600! dark:text-lime-500!",
                background && "rounded bg-muted",
            )}
        >
            {code}
        </span>
    );
};

export const InlineCode = ({
    code,
    lang = "Shell-Unix-Generic",
    theme = "Dracula",
    loading = null,
}: HighlightCodeRequest & {
    /**
     * The content shown while loading. Setting to a string of 'content' will use the content as inline code until the code renders.
     */
    loading?: ReactNode | "content";
}): ReactNode => {
    const content = useHighlightedCode({
        req: {
            code,
            lang,
            theme,
            inline: true,
        },
    });
    if (!content) {
        if (loading === "content") {
            return (
                <span>
                    <code className="font-mono text-lime-600 dark:text-lime-500">
                        {code}
                    </code>
                </span>
            );
        }
        return loading;
    }
    return <span dangerouslySetInnerHTML={{ __html: content }} />;
};

InlineCode.displayName = "InlineCode";
