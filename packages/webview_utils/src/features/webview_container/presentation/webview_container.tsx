import React, { type CSSProperties, useEffect, useRef, type ReactNode } from "react";
import { cn } from "@/utils/cn";
import { setWebviewWindowBridgeFunctions } from "../state/swift_events/webview_swift_events";
import { LoadingComponent } from "@/shared_components/loading_component";
import { connect } from 'react-redux';
import { type MdxEditorAppState } from '#/webview_global_state/mdx_editor/store';
import {
    type ScreenDimensions,
    useScreenDimensions,
} from "@/state/hooks/use_screen_dimensions";
import { type AnyWebviewAction } from "@/utils/types/any_window_event";
import { getSmallestSizableBreakpointByWidth } from "#/mdx/embeddable_mdx_components/grid/embeddable_responsive_grid_props";
import { setEditorView } from "#/webview_global_state/mdx_editor/state/editor_state_slice";
import { setSize } from "#/webview_global_state/container/webview_container_global_state/webview_container_slice";
import { SplitviewEditorDomIds, SizableOption, type WebviewContainerState, EditorView } from "@/code_gen/typeshare/fluster_core_utilities";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { useDispatch } from "react-redux";
import { useMediaQuery } from "react-responsive";

interface WebViewContainerProps {
    children: ReactNode;
    className?: string;
    style?: CSSProperties;
    contentContainerClasses?: string;
    /** If shrinkHeight = true, will shrink to fit-content to allow window to resize to match content */
    shrinkHeight?: boolean;
    broadcastHeightKey?: AnyWebviewAction;
    /** An optional function that can accept the actual screen dimensions sent by swift and returns another screen dimensions that the webview will bind it's size to. */
    screenDimensionCalculator?: (
        actualDimensions: ScreenDimensions,
    ) => CSSProperties;
}

setWebviewWindowBridgeFunctions();

/** A utility function intended to have a shared place to get this element since this webview keeps breaking. */
export const getParentContentWrapper = () =>
    document.getElementById("webview-content-wrapper");


const connector = connect((state: MdxEditorAppState) => ({
    size: state.container.size
}))

export const WebViewContainer = connector(({
    className,
    size,
    children,
    shrinkHeight,
    style,
    contentContainerClasses,
    screenDimensionCalculator,
}: WebViewContainerProps & Pick<WebviewContainerState, "size">): ReactNode => {
    /* const updateDocSizeTimer = useRef<NodeJS.Timeout | null>(null); */
    const container = useRef<HTMLDivElement>(null);
    const dimensions = useScreenDimensions(screenDimensionCalculator);
    const dispatch = useDispatch()

    // -- Size --
    const onResize = (): void => {
        const em = document.getElementById(SplitviewEditorDomIds.MdxPreview)
        if (!em) {
            console.warn("Could not find mdx-preview container")
            return
        } else {
            const width = em.getBoundingClientRect().width
            const smallestSize = getSmallestSizableBreakpointByWidth(width) ?? SizableOption.Full
            if (smallestSize !== size) {
                dispatch(setSize(smallestSize))
            }
        }
    }
    useEventListener("main-panel-resize", () => {
        onResize()
    })
    useEffect(() => {
        onResize()
        window.addEventListener('resize', onResize)
        return () => {
            window.removeEventListener("resize", onResize)
        }
    }, [])

    // -- View --
    const isLandscape = useMediaQuery({
        orientation: "landscape",
    });

    useEffect(() => {
        dispatch(setEditorView(isLandscape ? EditorView.Splitview : EditorView.PreviewOnly))
    }, [isLandscape])

    return (
        <div
            id="webview-container"
            className={cn(
                "max-w-screen w-screen",
                shrinkHeight ? "h-fit" : "h-fit min-h-screen",
                className,
            )}
            style={{
                ...style,
                ...(screenDimensionCalculator &&
                    dimensions && {
                    ...dimensions,
                }),
            }}
        >
            <div
                id="webview-content-wrapper"
                className={cn(
                    "w-full load-hide",
                    shrinkHeight ? "h-fit" : "h-fit min-h-screen",
                    contentContainerClasses,
                )}
                ref={container}
            >
                {children}
            </div>
            <div
                id="loading-indicator"
                className="w-screen h-screen flex flex-col justify-center items-center loading load-show fixed top-0 left-0 right-0 bottom-0"
            >
                <LoadingComponent className="w-fit h-fit" />
            </div>
        </div>
    );
});

WebViewContainer.displayName = "WebViewContainer";
