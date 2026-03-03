import { AnyWebviewAction } from "@/utils/types/any_window_event";
import { HTMLProps } from "react";
import { ComponentMapItem } from "../methods/get_component_map";

export interface MdxContentProps extends HTMLProps<HTMLDivElement> {
    mdx: string;
    className?: string;
    abortIfNoMath?: boolean;
    debounceTimeout?: number;
    showWebviewAction?: AnyWebviewAction
    additionalComponents?: ComponentMapItem[]
    lockToEditorScroll?: boolean
}
