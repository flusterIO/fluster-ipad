import { useCodeEditorContext } from "#/editor/code_editor/state/code_editor_provider";
import React, { type ReactNode } from "react";
import { MdxContent } from "./mdx_content";
import { useMediaQuery } from "react-responsive";
import { cn } from "@/utils/cn";

export const MdxEditorPreview = (): ReactNode => {
    const { value } = useCodeEditorContext();

    const isEditorView = useMediaQuery({
        orientation: "landscape",
    });

    if (value === "") {
        return (
            <div className="w-full h-full flex flex-col justify-center items-center">
                <div
                    className="text-xl font-semibold"
                    style={{
                        color: "hsl(var(--foreground))",
                    }}
                >
                    Content Empty
                </div>
            </div>
        );
    }
    return (
        <MdxContent
            removeGrayMatter
            className={cn(
                "block max-h-full",
                isEditorView ? "px-4 pt-4" : "px-8 pt-6",
            )}
            mdx={value}
        />
    );
};

MdxEditorPreview.displayName = "MdxEditorPreview";
