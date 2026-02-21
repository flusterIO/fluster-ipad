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
                "w-full p-2 rounded-tl rounded-tr relative cursor-pointer z-[1] min-h-[42px] font-semibold grid place-items-center",
                getTitleVariantClasses(props.type)
            )}
            style={{
                gridTemplateColumns: `48px 1fr`
            }}
        >
            <AdmonitionVariantIcon
                className={admonitionTitleIconClasses}
                variant={props.type}
            />
            <span className="inline-block w-full">{props.children}</span>
        </div>
    );
};

AdmonitionTitle.displayName = "AdmonitionTitle";

export default AdmonitionTitle;

