import { cn } from "@/utils/cn";
import React, { HTMLProps, useEffect, type ReactNode } from "react";
import { useDebounceMdxParse } from "../hooks/use_debounce_mdx_parse";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { useLocalStorage } from "@/state/hooks/use_local_storage";

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
    debounceTimeout = 300,
    ...props
}: MdxContentProps): ReactNode => {
    const { Component, setValue } = useDebounceMdxParse(
        undefined,
        debounceTimeout,
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

    /* NOTE: This works, but it causes a weird sizing issues with a white bar on Apple's end. I'll come back to scroll restoration later. */
    /* useEffect(() => { */
    /*     if (props.id && props.scrollPositionKey) { */
    /*         const element = document.getElementById(props.id); */
    /*         if (!element) { */
    /*             return; */
    /*         } */
    /*         loadScrollPosition(element, props.scrollPositionKey); */
    /*     } */
    /* }, [value]); */

    const classes = cn(
        "mdx-content block pb-12 overflow-y-hidden h-auto max-h-fit prose dark:prose-invert prose-p:text-foreground prose-code:before:content-none prose-code:after:content-none prose-code:bg-[var(--shiki-light-bg)] dark:prose-code:bg-[var(--shiki-dark-bg)] [&_code_*]:text-[var(--shiki-light)] dark:[&_code_*]:text-[var(--shiki-dark)] w-full max-w-full @container/mdx prose-code:p-2 prose-pre:bg-transparent dark:prose-pre:bg-transparent",
        fontSizeClass,
        className,
    );

    if (abortIfNoMath && !mdx.includes("$")) {
        return (
            <div {...props} className={classes}>
                {mdx}
            </div>
        );
    }

    return <Component {...props} className={classes} />;
};

MdxContent.displayName = "MdxContent";
