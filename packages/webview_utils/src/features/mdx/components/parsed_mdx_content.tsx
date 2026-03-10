import React, { type HTMLProps, useEffect, useRef, useState } from "react";
import { useComponentMap } from "../hooks/use_component_map";
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
}

export const ParsedMdxContent = ({
    MdxContentComponent,
    raw,
    container,
    scrollPositionKey,
    showWebviewHandler,
    additionalComponents
}: Props) => {
    /* const componentMap = useComponentMap(raw, additionalComponents); */
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
    return (
        <div {...container}>
            <MdxContentComponent components={components} />
        </div>
    );
};
