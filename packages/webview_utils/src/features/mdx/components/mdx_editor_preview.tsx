import { useCodeEditorContext } from "#/editor/code_editor/state/code_editor_provider";
import React, { type ReactNode } from "react";
import { MdxContent } from "./mdx_content";

export const MdxEditorPreview = (): ReactNode => {
    const { value } = useCodeEditorContext();
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
            className="p-6 max-h-full contents prose dark:prose-invert"
            mdx={value}
        />
    );
};

MdxEditorPreview.displayName = "MdxEditorPreview";
