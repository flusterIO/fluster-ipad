import { cn } from "@/utils/cn";
import React, { HTMLProps, useEffect, useState, type ReactNode } from "react";
import { useDebounceMdxParse } from "../hooks/use_debounce_mdx_parse";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { useLocalStorage } from "@/state/hooks/use_local_storage";

export interface MdxContentProps extends HTMLProps<HTMLDivElement> {
    mdx: string;
    className?: string;
    abortIfNoMath?: boolean;
    debounceTimeout?: number;
    removeGrayMatter?: boolean;
}

export const MdxContent = ({
    mdx,
    className,
    abortIfNoMath,
    debounceTimeout = 300,
    removeGrayMatter,
    ...props
}: MdxContentProps): ReactNode => {
    const { Component, setValue } = useDebounceMdxParse(
        undefined,
        debounceTimeout,
    );
    const [fontSizeClass, setFontSizeClass] = useLocalStorage(
        "webview-font-class",
        "prose-base",
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

    const classes = cn(
        "mdx-content block pb-12 prose dark:prose-invert prose-p:text-foreground prose-code:before:content-none prose-code:after:content-none prose-code:bg-[--shiki-light-bg] dark:prose-code:bg-[--shiki-dark-bg] [&_code_*]:text-[--shiki-light] dark:[&_code_*]:text-[--shiki-dark] w-full max-w-full @container/mdx prose-code:p-2 prose-pre:bg-transparent dark:prose-pre:bg-transparent",
        fontSizeClass,
        className,
    );

    if (abortIfNoMath && !mdx.includes("$")) {
        return <div className={classes}>{mdx}</div>;
    }

    return <Component {...props} className={classes} />;
};

MdxContent.displayName = "MdxContent";
