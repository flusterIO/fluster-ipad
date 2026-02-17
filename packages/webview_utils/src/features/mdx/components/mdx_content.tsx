import { cn } from "@/utils/cn";
import React, { HTMLProps, useEffect, useId, type ReactNode } from "react";
import { useDebounceMdxParse } from "../hooks/use_debounce_mdx_parse";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { useLocalStorage } from "@/state/hooks/use_local_storage";
import type { AnyWebviewAction } from "@/utils/types/any_window_event";
import { ComponentMapItem } from "../methods/get_component_map";
import { mdxClasses } from "./inline_mdx_classes";

export interface MdxContentProps extends HTMLProps<HTMLDivElement> {
    mdx: string;
    className?: string;
    abortIfNoMath?: boolean;
    debounceTimeout?: number;
    showWebviewAction?: AnyWebviewAction
    additionalComponents?: ComponentMapItem[]
}

export const MdxContent = ({
    mdx,
    className,
    abortIfNoMath,
    debounceTimeout = 300,
    showWebviewAction,
    additionalComponents,
    ...props
}: MdxContentProps): ReactNode => {
    const id = useId()
    const { Component, setValue } = useDebounceMdxParse(
        undefined,
        debounceTimeout,
        id,
        showWebviewAction,
        additionalComponents
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

    const classes = cn(
        "mdx-content block overflow-y-hidden h-auto max-h-fit w-full max-w-full @container/mdx prose-code:p-2 prose-pre:bg-transparent dark:prose-pre:bg-transparent",
        mdxClasses,
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
