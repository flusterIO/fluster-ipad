import { cn } from "@/utils/cn";
import React, { HTMLProps, useEffect, type ReactNode } from "react";
import { useDebounceMdxParse } from "../hooks/use_debounce_mdx_parse";

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

    useEffect(() => {
        setValue(mdx);
        /* eslint-disable-next-line  -- I hate that this rule applies to functions... */
    }, [mdx]);

    if (abortIfNoMath && !mdx.includes("$")) {
        return mdx;
    }

    return (
        <Component
            {...props}
            className={cn(
                "mdx-content prose dark:prose-invert prose-p:text-foreground prose-code:before:content-none prose-code:after:content-none prose-code:bg-[--shiki-light-bg] dark:prose-code:bg-[--shiki-dark-bg] [&_code_*]:text-[--shiki-light] dark:[&_code_*]:text-[--shiki-dark] w-full  max-w-full @container/mdx prose-code:p-2 prose-pre:bg-transparent dark:prose-pre:bg-transparent",
                className,
            )}
        />
    );
};

MdxContent.displayName = "MdxContent";
