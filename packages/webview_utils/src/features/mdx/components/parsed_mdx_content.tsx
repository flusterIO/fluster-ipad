import React, { HTMLProps, useEffect } from "react";
import { ErrorBoundary } from "react-error-boundary";
import { useComponentMap } from "../hooks/use_component_map";
import { H2 } from "../../../core/shared_components/typography/typography";
import type { MDXContent } from "mdx/types";

interface Props {
    MdxContentComponent: MDXContent;
    raw: string;
    container?: HTMLProps<HTMLDivElement>;
    scrollPositionKey?: string
}

export const ParsedMdxContent = ({
    MdxContentComponent,
    raw,
    container,
    scrollPositionKey
}: Props) => {
    const componentMap = useComponentMap(raw);
    useEffect(() => {
        window.dispatchEvent(new CustomEvent("mdx-content-loaded", {
            detail: scrollPositionKey
        }))
    }, [MdxContentComponent, scrollPositionKey])
    return (
        <div {...container}>
            <ErrorBoundary
                onError={(e) => {
                    console.error(`Mdx Error: ${e}`);
                }}
                fallback={
                    <div className="w-full h-full flex flex-col justify-center items-center">
                        <H2>Oh no</H2>
                        <p className="text-foreground/80 max-w-[540px] text-center">
                            This note can not be parsed successfully. There is likely a syntax
                            error in your note.{" "}
                        </p>
                    </div>
                }
            >
                <MdxContentComponent components={componentMap} />
            </ErrorBoundary>
        </div>
    );
};
