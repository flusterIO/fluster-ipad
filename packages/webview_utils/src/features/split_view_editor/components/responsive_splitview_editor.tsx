import { CodeEditorProvider } from "#/editor/code_editor/state/code_editor_provider";
import React, { type ReactNode } from "react";
import { useMediaQuery } from "react-responsive";
import { SplitViewEditorInner } from "./split_view_editor";
import { MdxEditorPreviewOnly } from "#/mdx/components/mdx_content_preview_only";
import { LoadingComponent } from "@/shared_components/loading_component";


export const ResponsiveSplitViewEditor = (): ReactNode => {
    const isLandscape = useMediaQuery({
        orientation: "landscape",
    });
    return (
        <>
            <CodeEditorProvider>
                {isLandscape ? <SplitViewEditorInner /> : <MdxEditorPreviewOnly />}
                <div className="w-full h-full flex flex-col justify-center items-center p-8 loading-main-only hide-desktop">
                    <LoadingComponent />
                </div>
                {/* <MdxParsingErrorIndicator /> */}
            </CodeEditorProvider>
        </>
    );
};

ResponsiveSplitViewEditor.displayName = "ResponsiveSplitViewEditor";
