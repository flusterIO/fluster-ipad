import React, { type ReactNode } from "react";
import {
    useHighlightedCode,
    type HighlightCodeRequest,
} from "../state/hooks/use_highlighted_code";

export const InlineCode = ({
    code,
    lang = "Shell-Unix-Generic",
    theme = "Dracula",
    loading = null,
}: HighlightCodeRequest & { loading?: ReactNode }): ReactNode => {
    const content = useHighlightedCode({
        req: {
            code,
            lang,
            theme,
            inline: true,
        },
    });
    if (!content) {
        return loading;
    }
    return <span dangerouslySetInnerHTML={{ __html: content }} />;
};

InlineCode.displayName = "InlineCode";
