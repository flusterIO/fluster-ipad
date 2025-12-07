import { usePersistMdxPreviewScroll } from "@/state/hooks/use_persist_scroll";
import { type ReactNode } from "react";

export const MdxPreviewScrollPersistor = ({
    debounce = 500
}: {
    debounce?: number
}): ReactNode => {
    usePersistMdxPreviewScroll(debounce);
    return null;
};

MdxPreviewScrollPersistor.displayName = "MdxPreviewScrollPersistor";
