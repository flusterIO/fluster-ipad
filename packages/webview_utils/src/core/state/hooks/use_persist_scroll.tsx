import { useEffect, useLayoutEffect } from "react";
import { useEventListener } from "./use_event_listener";
import { useMediaQuery } from "react-responsive";
import { SplitviewEditorWebviewEvents } from "@/code_gen/typeshare/fluster_core_utilities";

export const MDX_EDITOR_PREVIEW_SCROLL_LANDSCAPE_KEY =
    "mdx-preview-landscape-scroll";
export const MDX_EDITOR_PREVIEW_SCROLL_PORTRAIT_KEY =
    "mdx-preview-portrait-scroll";
export const MDX_EDITOR_PREVIEW_ID_PORTRAIT = "mdx-preview-portrait";
export const MDX_EDITOR_PREVIEW_ID_LANDSCAPE = "mdx-preview-landscape";

export const loadScrollPosition = (
    element: HTMLElement,
    storageKey: string,
) => {
    const savedPosition = localStorage.getItem(storageKey);
    console.log("savedPosition: ", savedPosition);
    if (savedPosition) {
        element.scrollTop = parseInt(savedPosition);
    }
};

export const usePersistMdxPreviewScroll = () => {
    const isLandscape = useMediaQuery({
        orientation: "landscape",
    });
    const storageKey = isLandscape
        ? MDX_EDITOR_PREVIEW_SCROLL_LANDSCAPE_KEY
        : MDX_EDITOR_PREVIEW_SCROLL_PORTRAIT_KEY;

    useLayoutEffect(() => {
        const elementId = isLandscape
            ? MDX_EDITOR_PREVIEW_ID_LANDSCAPE
            : MDX_EDITOR_PREVIEW_ID_PORTRAIT;
        const element = document.getElementById(elementId);
    }, [isLandscape]);
    useEffect(() => {
        const elementId = isLandscape
            ? MDX_EDITOR_PREVIEW_ID_LANDSCAPE
            : MDX_EDITOR_PREVIEW_ID_PORTRAIT;
        const element = document.getElementById(elementId);

        // Safety check: if element doesn't exist (yet), stop here
        if (!element) return;
        loadScrollPosition(element, storageKey);
        // 1. Restore: Set the element's scrollTop
    }, [isLandscape]);
    useEventListener(SplitviewEditorWebviewEvents.ResetPreviewScrollPosition, () => {
        window.localStorage.removeItem(storageKey);
    });
};
