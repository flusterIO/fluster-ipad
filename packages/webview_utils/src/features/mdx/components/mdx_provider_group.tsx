import React, { type ReactNode } from "react";
import { MdxImageProvider } from "src/desktop/features/media/images/state/img_provider";
import { ImagePageListener } from "../state/image_page_listener";

interface MdxProviderGroupProps {
    children: ReactNode;
}

export const MdxProviderGroup = ({
    children,
}: MdxProviderGroupProps): ReactNode => {
    return (
        <MdxImageProvider>
            <ImagePageListener />
            {/* <TaskManagerTimerHandler /> */}
            {children}
        </MdxImageProvider>
    );
};

MdxProviderGroup.displayName = "MdxProviderGroup";
