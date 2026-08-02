import { type ReactNode } from "react";
import { createPortal } from "react-dom";

interface PortalProps {
    children: ReactNode;
    querySelector?: string;
}

export const Portal = ({
    children,
    querySelector = "body",
}: PortalProps): ReactNode => {
    const em = document.querySelector(querySelector);
    if (em) {
        return createPortal(children, em);
    } else {
        return null;
    }
};

Portal.displayName = "Portal";
