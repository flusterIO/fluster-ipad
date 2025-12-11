import React, { type ReactNode } from "react";
import AdmonitionVariantIcon from "./variant_icon";
import { getTitleVariantClasses } from "./utils";
import { AdmonitionTitleProps } from "./types";
import { cn } from "@/utils/cn";

interface Props extends AdmonitionTitleProps {
    children: ReactNode;
}

export const admonitionTitleIconClasses =
    "inline-block w-6 h-6 place-self-center";

const AdmonitionTitle = (props: Props): ReactNode => {
    return (
        <div
            className={cn(
                "w-full pl-2 pr-4 pt-2 rounded-tl rounded-tr relative z-[1] min-h-[42px] font-semibold",
                getTitleVariantClasses(props.type)
            )}
        >
            <AdmonitionVariantIcon
                className={admonitionTitleIconClasses}
                variant={props.type}
            />
            <span className="inline-block w-[calc(100%-4rem)]">{props.children}</span>
        </div>
    );
};

AdmonitionTitle.displayName = "AdmonitionTitle";

export default AdmonitionTitle;
