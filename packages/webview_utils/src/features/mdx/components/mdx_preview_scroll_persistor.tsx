import { usePersistMdxPreviewScroll } from "@/state/hooks/use_persist_scroll";
import { type ReactNode } from "react";

export const MdxPreviewScrollPersistor = (): ReactNode => {
    usePersistMdxPreviewScroll();
    return null;
};

MdxPreviewScrollPersistor.displayName = "MdxPreviewScrollPersistor";
