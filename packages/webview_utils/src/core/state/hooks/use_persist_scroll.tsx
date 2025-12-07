import { useEffect, useRef } from "react";
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

export const usePersistMdxPreviewScroll = (debounce: number = 500) => {
    const isLandscape = useMediaQuery({
        orientation: "landscape",
    });
    const timer = useRef<NodeJS.Timeout | null>(null);
    const storageKey = isLandscape
        ? MDX_EDITOR_PREVIEW_SCROLL_LANDSCAPE_KEY
        : MDX_EDITOR_PREVIEW_SCROLL_PORTRAIT_KEY;

    const handleScroll = (e: Event): void => {
        if (timer.current) {
            clearTimeout(timer.current)
        }
        timer.current = setTimeout(() => window.localStorage.setItem(storageKey, (e.target as HTMLElement).scrollTop.toString()), debounce)
    }

    useEffect(() => {
        const elementId = isLandscape
            ? MDX_EDITOR_PREVIEW_ID_LANDSCAPE
            : MDX_EDITOR_PREVIEW_ID_PORTRAIT;
        const element = document.getElementById(elementId);
        if (element) {
            loadScrollPosition(element, storageKey);
            element.addEventListener("scroll", handleScroll)
            return () => element.removeEventListener("scroll", handleScroll)
        } else {
            console.error(`Could not find element with the id ${elementId} to persist scroll.`)
        }
        /* eslint-disable-next-line  -- I hate this rule. */
    }, [isLandscape, storageKey]);
    useEventListener(SplitviewEditorWebviewEvents.ResetPreviewScrollPosition, () => {
        window.localStorage.removeItem(storageKey);
    });
};
