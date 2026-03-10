import React, { type HTMLProps, useEffect, useRef, useState } from "react";
import { type MDXComponents, type MDXContent } from "mdx/types";
import { sendToSwift } from "@/utils/bridge/send_to_swift";
import { type AnyWebviewAction } from "@/utils/types/any_window_event";
import { type ComponentMapItem, getComponentMap } from "../methods/get_component_map";
import { LoadingComponent } from "@/shared_components/loading_component";

interface Props {
    MdxContentComponent: MDXContent;
    raw: string;
    container?: HTMLProps<HTMLDivElement>;
    scrollPositionKey?: string;
    showWebviewHandler?: AnyWebviewAction;
    additionalComponents?: ComponentMapItem[]
    asMain?: boolean
}

export const ParsedMdxContent = ({
    MdxContentComponent,
    raw,
    container,
    scrollPositionKey,
    showWebviewHandler,
    additionalComponents,
    asMain
}: Props) => {
    const [components, setComponents] = useState<MDXComponents | null>(null)
    useEffect(() => {
        const handleComponentMap = async (): Promise<void> => {
            setComponents(null)
            const res = await getComponentMap(raw, additionalComponents)
            setComponents(res)
        }
        handleComponentMap()
    }, [raw])
    useEffect(() => {
        window.dispatchEvent(new CustomEvent("mdx-content-loaded", {
            detail: {
                scrollPositionKey
            }
        }))
        document.body.classList.remove("loading")
        document.body.classList.remove("loading-main")
        if (showWebviewHandler) {
            sendToSwift(showWebviewHandler);
        }

    }, [MdxContentComponent, scrollPositionKey])
    if (!components) {
        return (
            <div className="w-full h-full flex flex-col justify-center items-center">
                <LoadingComponent />
            </div>
        )
    }
    if (asMain) {
        return (
            <main {...container}>
                <MdxContentComponent components={components} />
            </main >
        )
    }
    return (
        <div {...container}>
            <MdxContentComponent components={components} />
        </div>
    );
};
