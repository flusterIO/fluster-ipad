import { RefObject, useEffect, useRef } from "react";
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

export const usePersistMdxPreviewScroll = (
    ref: RefObject<HTMLElement | null>,
    debounce: number = 500,
    /// An optional id that must match the contentLoadedId field passed to the useDebounceMdxParse hook.
    mdxContentId: string = "mdx-content"
) => {
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
    const handleScrollSetup = (): void => {
        const elementId = isLandscape
            ? MDX_EDITOR_PREVIEW_ID_LANDSCAPE
            : MDX_EDITOR_PREVIEW_ID_PORTRAIT;
        const element = ref.current ?? document.getElementById(elementId);
        if (element) {
            loadScrollPosition(element, storageKey);
            element.removeEventListener("scroll", handleScroll)
            element.addEventListener("scroll", handleScroll)
        }
    }

    useEffect(() => {
        handleScrollSetup()
        /* eslint-disable-next-line -- I hate this rule but I'm too lazy to turn it off. */
    }, [isLandscape, storageKey]);

    useEventListener("mdx-content-loaded", (e) => {
        if (e.detail === mdxContentId) {
            handleScrollSetup()
        }
    })

    useEventListener(SplitviewEditorWebviewEvents.ResetPreviewScrollPosition, () => {
        window.localStorage.removeItem(storageKey);
    });
};
