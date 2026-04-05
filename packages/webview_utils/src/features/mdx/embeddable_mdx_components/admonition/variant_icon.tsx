import React, { type ReactNode } from "react";
import { BadgeAlert, BellRing, CircleCheck, Hand, Info, Microscope, TriangleAlertIcon } from "lucide-react";
import { Emphasis } from "../schemas/emphasis_schema";

interface AdmonitionVariantIconProps {
    variant: Emphasis;
    className?: string;
}

const AdmonitionVariantIcon = (
    props: AdmonitionVariantIconProps
): NonNullable<ReactNode> => {
    switch (props.variant) {
        case Emphasis.Info:
            return <Info className={props.className} />;
        case Emphasis.Warn:
            return <TriangleAlertIcon className={props.className} />;
        case Emphasis.Error:
            return <Hand className={props.className} />;
        case Emphasis.Success:
            return <CircleCheck className={props.className} />;
        case Emphasis.Primary:
            return <Info className={props.className} />;
        case Emphasis.Important:
            return <BadgeAlert className={props.className} />
        case Emphasis.Research:
            return <Microscope className={props.className} />
        case Emphasis.Highlight:
            return <BellRing className={props.className} />
        case Emphasis.Card:
            return <Info className={props.className} />
    }
};

AdmonitionVariantIcon.displayName = "AdmonitionVariantIcon";

export default AdmonitionVariantIcon;
