import { useCodeEditorContext } from "#/editor/code_editor/state/code_editor_provider";
import React, { HTMLProps, type ReactNode } from "react";
import { MdxContent, MdxContentProps } from "./mdx_content";
import { useMediaQuery } from "react-responsive";
import { cn } from "@/utils/cn";
import { LoadingComponent } from "@/shared_components/loading_component";

export type MdxEditorPreviewProps = HTMLProps<HTMLDivElement> &
    Pick<MdxContentProps, "scrollPositionKey">;

export const MdxEditorPreview = ({
    className,
    ...props
}: MdxEditorPreviewProps): ReactNode => {
    const { value, parsedValue } = useCodeEditorContext();

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

    if (!parsedValue || parsedValue === "") {
        return (
            <div className="w-full h-full flex flex-col justify-center items-center">
                <LoadingComponent />
            </div>
        );
    }
    return (
        <MdxContent
            id="mdx-preview"
            {...props}
            className={cn(
                "max-w-[1080px]",
                isEditorView ? "px-6 pt-4" : "px-8 pt-6 max-h-screen overflow-y-auto",
                className,
            )}
            mdx={parsedValue}
            onScrollEnd={(e) => console.log("e: ", e)}
        />
    );
};

MdxEditorPreview.displayName = "MdxEditorPreview";
