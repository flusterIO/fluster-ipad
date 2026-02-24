import React, { type ReactNode } from "react";
import { BadgeAlert, CircleCheck, Hand, Info, Microscope, TriangleAlertIcon } from "lucide-react";
import { Emphasis } from "../schemas/emphasis_schema";

interface AdmonitionVariantIconProps {
    variant: Emphasis;
    className?: string;
}

const AdmonitionVariantIcon = (
    props: AdmonitionVariantIconProps
): ReactNode => {
    switch (props.variant) {
        case "info":
            return <Info className={props.className} />;
        case "warn":
            return <TriangleAlertIcon className={props.className} />;
        case "error":
            return <Hand className={props.className} />;
        case "success":
            return <CircleCheck className={props.className} />;
        case "primary":
            return <Info className={props.className} />;
        case "important":
            return <BadgeAlert className={props.className} />
        case "research":
            return <Microscope className={props.className} />
    }
};

AdmonitionVariantIcon.displayName = "AdmonitionVariantIcon";

export default AdmonitionVariantIcon;
