import { useCodeEditorContext } from "#/editor/code_editor/state/code_editor_provider";
import React, { HTMLProps, type ReactNode } from "react";
import { MdxContent, MdxContentProps } from "./mdx_content";
import { useMediaQuery } from "react-responsive";
import { cn } from "@/utils/cn";

export type MdxEditorPreviewProps = HTMLProps<HTMLDivElement> &
    Pick<MdxContentProps, "scrollPositionKey">;

export const MdxEditorPreview = ({
    className,
    ...props
}: MdxEditorPreviewProps): ReactNode => {
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
            id="mdx-preview"
            removeGrayMatter
            {...props}
            className={cn(
                isEditorView ? "px-4 pt-4" : "px-8 pt-6 max-h-screen overflow-y-auto",
                className,
            )}
            mdx={value}
        /* onScroll={(e) => { */
        /*     if (props.scrollPositionKey) { */
        /*         if (timer.current) { */
        /*             clearTimeout(timer.current); */
        /*         } */
        /*         timer.current = setTimeout(() => { */
        /*             localStorage.setItem( */
        /*                 props.scrollPositionKey!, */
        /*                 (e.target as HTMLDivElement).scrollTop.toString(), */
        /*             ); */
        /*         }, 250); */
        /*     } */
        /* }} */
        />
    );
};

MdxEditorPreview.displayName = "MdxEditorPreview";
