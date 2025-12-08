import React, { HTMLProps, useEffect } from "react";
import { ErrorBoundary } from "react-error-boundary";
import { useComponentMap } from "../hooks/use_component_map";
import { H2 } from "../../../core/shared_components/typography/typography";
import type { MDXContent } from "mdx/types";
import { useMediaQuery } from "react-responsive";
import { sendToSwift } from "@/utils/bridge/send_to_swift";
import { AnyWebviewAction } from "@/utils/types/any_window_event";

interface Props {
    MdxContentComponent: MDXContent;
    raw: string;
    container?: HTMLProps<HTMLDivElement>;
    scrollPositionKey?: string
    showWebviewHandler?: AnyWebviewAction
}

export const ParsedMdxContent = ({
    MdxContentComponent,
    raw,
    container,
    scrollPositionKey,
    showWebviewHandler
}: Props) => {
    const componentMap = useComponentMap(raw);
    const isLandscape = useMediaQuery({
        orientation: "landscape",
    });
    useEffect(() => {
        window.dispatchEvent(new CustomEvent("mdx-content-loaded", {
            detail: scrollPositionKey
        }))
        document.body.classList.remove("loading")
        document.body.classList.remove("loading-main")
        if (!isLandscape && showWebviewHandler) {
            sendToSwift(showWebviewHandler);
        }
        /* eslint-disable-next-line  -- Don't actually want this to run on change of orientation... just the original */
    }, [MdxContentComponent, scrollPositionKey])
    return (
        <div {...container}>
            <ErrorBoundary
                onError={(e) => {
                    console.error(`Mdx Error: ${e}`);
                }}
                fallback={
                    <div className="w-full h-full flex flex-col justify-center items-center">
                        <H2>Oh no</H2>
                        <p className="text-foreground/80 max-w-[540px] text-center">
                            This note cannot be parsed successfully. There is likely a syntax
                            error in your note.{" "}
                        </p>
                    </div>
                }
            >
                <MdxContentComponent components={componentMap} />
            </ErrorBoundary>
        </div>
    );
};
