import { cn } from "@/utils/cn";
import React, { HTMLProps, useEffect, type ReactNode } from "react";
import { useDebounceMdxParse } from "../hooks/use_debounce_mdx_parse";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { useLocalStorage } from "@/state/hooks/use_local_storage";

const loadScrollPosition = (
    element: HTMLElement,
    storageKey: string,
) => {
    const savedPosition = localStorage.getItem(storageKey);
    if (savedPosition) {
        element.scrollTop = parseInt(savedPosition);
    }
};

export interface MdxContentProps extends HTMLProps<HTMLDivElement> {
    mdx: string;
    className?: string;
    abortIfNoMath?: boolean;
    debounceTimeout?: number;
    scrollPositionKey?: string;
}

export const MdxContent = ({
    mdx,
    className,
    abortIfNoMath,
    scrollPositionKey,
    debounceTimeout = 300,
    ...props
}: MdxContentProps): ReactNode => {
    const { Component, setValue } = useDebounceMdxParse(
        undefined,
        debounceTimeout,
        scrollPositionKey
    );
    const [fontSizeClass, setFontSizeClass] = useLocalStorage(
        "webview-font-class",
        undefined,
        {
            deserializer(value) {
                return value;
            },
            serializer(value) {
                return value;
            },
            initializeWithValue: false,
        },
    );

    useEventListener("set-webview-font-size", (e) => {
        setFontSizeClass(e.detail);
    });

    useEffect(() => {
        setValue(mdx);
        /* eslint-disable-next-line  -- I hate that this rule applies to functions... */
    }, [mdx]);

    const handleScroll = (e: Event): void => {
        if (scrollPositionKey) {
            window.localStorage.setItem(scrollPositionKey, (e.target as HTMLElement).scrollTop.toString())
        }
    }

    const handleScrollSetup = (id: string, sk: typeof scrollPositionKey): void => {
        if (id && sk) {
            const em = document.getElementById(id);
            if (em) {
                loadScrollPosition(em, sk);
                em.removeEventListener("scroll", handleScroll)
                em.addEventListener("scroll", handleScroll)
            } else {
                console.log(`Can't find the fucking element...`);
            }
        }
    }


    useEventListener("mdx-content-loaded", (e) => {
        if (props.id && (e.detail === scrollPositionKey)) {
            handleScrollSetup(props.id, scrollPositionKey)
        }
    })

    const classes = cn(
        "mdx-content block pb-12 overflow-y-hidden h-auto max-h-fit prose dark:prose-invert prose-p:text-foreground prose-code:before:content-none prose-code:after:content-none prose-code:bg-[var(--shiki-light-bg)] dark:prose-code:bg-[var(--shiki-dark-bg)] [&_code_*]:text-[var(--shiki-light)] dark:[&_code_*]:text-[var(--shiki-dark)] w-full max-w-full @container/mdx prose-code:p-2 prose-pre:bg-transparent dark:prose-pre:bg-transparent",
        fontSizeClass,
        className,
    );

    if (abortIfNoMath && !mdx.includes("$")) {
        return (
            <div
                {...props}
                className={classes}
            >
                {mdx}
            </div>
        );
    }

    return <Component
        {...props}
        className={classes}
    />;
};

MdxContent.displayName = "MdxContent";
