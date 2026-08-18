import { rspc } from "@/app/rspc_client";
import { cn } from "@/utils/shad_utils";
import consola from "consola";
import React, { useEffect, useState, type ReactNode } from "react";

interface InlineCdrmContentProps {
    content: string;
    className?: string;
    em?: "span" | "div";
    inline?: boolean;
}

export const CdrmContent = ({
    className,
    content,
    inline,
    em,
}: InlineCdrmContentProps): ReactNode => {
    const { mutateAsync } = rspc.useMutation("cdrm.compile_cdrm", {});
    const [parsedContent, setParsedContent] = useState<null | string>(null);
    const _em = em ?? (inline ? "span" : "div");

    const parseContent = async (_content: string): Promise<void> => {
        const res = await mutateAsync({
            opts: {
                content: _content,
                target: "html",
                trusted: true,
                hide_components: [],
                modifiers: ["PreferInlineMarkdownSyntax"],
                note_id: null,
                ui_params: {
                    dark_mode: true,
                    font_scalar: 1,
                    math_font_scalar: 1.2,
                    syntax_theme: "Dracula",
                },
            },
        });
        setParsedContent(res.content);
    };

    useEffect(() => {
        parseContent(content).catch((err: unknown) => {
            consola.error("Error: ", err);
        });
    }, [content]);

    if (!parsedContent) {
        return _em === "span" ? <span /> : <div />;
    }

    if (_em === "span") {
        return (
            <span
                className={cn("inline-block w-full", className)}
                dangerouslySetInnerHTML={{ __html: parsedContent }}
            />
        );
    } else {
        return (
            <div
                className={cn("inline-block w-full", className)}
                dangerouslySetInnerHTML={{ __html: parsedContent }}
            />
        );
    }
};

CdrmContent.displayName = "InlineCdrmContent";
