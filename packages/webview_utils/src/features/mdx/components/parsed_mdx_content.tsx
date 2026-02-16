import React, { HTMLProps, useEffect } from "react";
import { ErrorBoundary } from "react-error-boundary";
import { useComponentMap } from "../hooks/use_component_map";
import type { MDXContent } from "mdx/types";
import { useMediaQuery } from "react-responsive";
import { sendToSwift } from "@/utils/bridge/send_to_swift";
import { type AnyWebviewAction } from "@/utils/types/any_window_event";
import { MdxParsingErrorComponent } from "./mdx_parsing_error_component";
import { ComponentMapItem } from "../methods/get_component_map";

interface Props {
    MdxContentComponent: MDXContent;
    raw: string;
    container?: HTMLProps<HTMLDivElement>;
    scrollPositionKey?: string;
    showWebviewHandler?: AnyWebviewAction;
    additionalComponents?: ComponentMapItem[]
}

export const ParsedMdxContent = ({
    MdxContentComponent,
    raw,
    container,
    scrollPositionKey,
    showWebviewHandler,
    additionalComponents
}: Props) => {
    const componentMap = useComponentMap(raw, additionalComponents);
    const isLandscape = useMediaQuery({
        orientation: "landscape",
    });
    useEffect(() => {
        window.dispatchEvent(new CustomEvent("mdx-content-loaded", {
            detail: {
                scrollPositionKey
            }
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
                FallbackComponent={MdxParsingErrorComponent}
            >
                <MdxContentComponent components={componentMap} />
            </ErrorBoundary>
        </div>
    );
};
