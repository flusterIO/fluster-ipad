import { cn } from "@/utils/cn";
import React, { type ReactNode } from "react";
import { MdxContent, type MdxContentProps } from "./mdx_content";
import { inlineMdxClasses } from "./inline_mdx_classes";


export const InlineMdxContent = (
    props: MdxContentProps & {
        abortIfNoMath?: boolean;
    },
): ReactNode => {
    if (!props.mdx || props.mdx.trim() === "") {
        return null;
    }
    if (
        props.abortIfNoMath &&
        typeof props.mdx === "string" &&
        !props.mdx?.includes("$")
    ) {
        return props.mdx;
    }
    return (
        <MdxContent
            {...props}
            className={cn(
                inlineMdxClasses,
                props.className,
            )}
        />
    );
};

InlineMdxContent.displayName = "InlineMdxContent";


export type WithInlineMdx = {
    InlineMdxContent: typeof InlineMdxContent
}
