import React, { type ReactNode } from "react";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { setMdxPreviewWindowMethods } from "./standalone_mdx_preview_swift_events";
import { MdxContent, MdxContentProps } from "../mdx_content";
import { useLocalStorage } from "@/state/hooks/use_local_storage";

setMdxPreviewWindowMethods();

declare global {
    interface WindowEventMap {
        "set-mdx-preview-content": CustomEvent<string>;
    }
}

export const MdxStandalonePreview = (
    props: Omit<MdxContentProps, "mdx">,
): ReactNode => {
    const [content, setContent] = useLocalStorage("mdx-preview-content", "", {
        deserializer: (s) => s,
        serializer: (s) => s,
        initializeWithValue: false,
    });

    useEventListener("set-mdx-preview-content", (e) => setContent(e.detail));
    return <MdxContent {...props} mdx={content} />;
};

MdxStandalonePreview.displayName = "MdxStandalonePreview";
