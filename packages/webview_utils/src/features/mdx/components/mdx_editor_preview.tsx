import { useCodeEditorContext } from "#/editor/code_editor/state/code_editor_provider";
import React, { type ReactNode } from "react";
import { MdxContent } from "./mdx_content";

export const MdxEditorPreview = (): ReactNode => {
    const { value } = useCodeEditorContext();
    if (value === "") {
        return (
            <div className="w-full h-full flex flex-col justify-center items-center">
                <div className="text-xl font-semibold">Content Empty</div>
            </div>
        );
    }
    return (
        <MdxContent
            removeGrayMatter
            className="p-6 max-h-full contents"
            mdx={value}
        />
    );
};

MdxEditorPreview.displayName = "MdxEditorPreview";
