import React, { type CSSProperties, type ReactNode } from "react";
import { cn } from "@/utils/cn";
import { setWebviewWindowBridgeFunctions } from "../state/swift_events/webview_swift_events";
import { LoadingComponent } from "@/shared_components/loading_component";
import {
    type ScreenDimensions,
} from "@/state/hooks/use_screen_dimensions";
import { type AnyWebviewAction } from "@/utils/types/any_window_event";
import { type GlobalAppState } from "#/webview_global_state/store";
import { MdxPreviewWebviewActions, type EditorBannerNotification, type WebviewContainerState } from "@/code_gen/typeshare/fluster_core_utilities";
import { connect } from "react-redux";
import { useEventListener } from "../../../core/state/hooks/use_event_listener";
import { useSendNotificationBanner } from "../../notifications/splitview_editor_notification_banner/send_splitview_notification_banner";
import { ConundrumWebEvents, CopyToClipboardSource } from "../../../core/code_gen/typeshare/conundrum";
import { sendToSwift } from "../../../core/utils/bridge/send_to_swift";


interface DictionaryLabelClickEventProps {
    noteId: string
}

declare global {
    interface WindowEventMap {
        [ConundrumWebEvents.DictionaryEntryLabelClick]: CustomEvent<DictionaryLabelClickEventProps>;
    }
}

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

const connector = connect((state: GlobalAppState) => ({
    fontSize: state.container.font_size
}))


export const WebViewContainer = connector(({
    className,
    children,
    shrinkHeight,
    style,
    fontSize,
    contentContainerClasses,
}: WebViewContainerProps & {
    fontSize: WebviewContainerState["font_size"]
}): ReactNode => {

    const fontSizeClasses: Record<typeof fontSize, string> = {
        base: "prose-base",
        large: "prose-lg",
        xxl: "prose-2xl",
        xl: "prose-xl",
        small: "prose-sm"
    }

    const showNotif = useSendNotificationBanner();

    useEventListener(ConundrumWebEvents.CopyToClipboard, (e) => {
        type X = Omit<EditorBannerNotification, "id">;
        const message = ({
            [CopyToClipboardSource.EmojiName]: {
                title: "Success",
                body: "Your emoji name has been successfully copied.",
                timeout: 3000
            }
        } satisfies Partial<Record<CopyToClipboardSource, X>>)[e.detail.source];
        /* eslint-disable-next-line  -- It's **not** always truthy you dumb ****. */
        if (message) {
            showNotif(message)
        }
    })


    useEventListener(ConundrumWebEvents.DictionaryEntryLabelClick, (e) => {
        const noteId = e.detail.noteId;
        if (noteId) {
            sendToSwift(MdxPreviewWebviewActions.ViewNoteById, noteId)
        }
    })


    return (
        <div
            id="webview-container"
            className={cn(
                "max-w-screen w-screen",
                shrinkHeight ? "h-fit" : "h-fit min-h-screen",
                className,
            )}
            style={style}
        >
            <div
                id="webview-content-wrapper"
                className={cn(
                    "w-full load-hide",
                    shrinkHeight ? "h-fit" : "h-fit min-h-screen",
                    fontSizeClasses[fontSize],
                    contentContainerClasses,
                )}
            >
                {children}
            </div>
            <div
                id="loading-indicator"
                className="w-screen h-screen flex flex-col justify-center items-center loading load-show fixed top-0 left-0 right-0 bottom-0"
            >
                <LoadingComponent className="w-fit h-fit" />
            </div>
        </div >
    );
});

WebViewContainer.displayName = "WebViewContainer";
