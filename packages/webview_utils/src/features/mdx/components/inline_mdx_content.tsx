import { cn } from "@/utils/cn";
import React, { type ReactNode } from "react";
import { MdxContent, type MdxContentProps } from "./mdx_content";

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
                "[&_p]:mb-0 [&_p]:mt-0 [&_p]:font-normal [&>p]:inline",
                props.className,
            )}
        />
    );
};

InlineMdxContent.displayName = "InlineMdxContent";
