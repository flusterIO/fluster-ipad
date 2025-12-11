import React, { type ReactNode } from "react";
import AdmonitionVariantIcon from "./variant_icon";
import { AdmonitionTitleProps } from "./types";
import { getTitleVariantClasses } from "./utils";
import { ChevronUp as ChevronIcon } from "lucide-react";
import { motion } from "framer-motion";
import { admonitionTitleIconClasses } from "./admonition_title";
import { cn } from "@/utils/cn";

const ChevronUp = motion.create(ChevronIcon);

interface Props extends AdmonitionTitleProps {
    open: boolean;
    setOpen: (newOpen: boolean) => void;
    children: ReactNode;
}

const FoldableAdmonitionTitle = (props: Props): ReactNode => {
    return (
        <div
            className={cn(
                "w-full p-2 rounded-tl rounded-tr relative cursor-pointer z-[1] min-h-[42px] font-semibold grid place-items-center",
                getTitleVariantClasses(props.type)
            )}
            style={{
                gridTemplateColumns: `48px 1fr 32px`
            }}
            onClick={() => props.setOpen(!props.open)}
        >
            <AdmonitionVariantIcon
                className={admonitionTitleIconClasses}
                variant={props.type}
            />
            <span className="inline-block w-full">
                {props.children}
            </span>
            <ChevronUp
                className="place-self-center"
                /* className="absolute top-2 right-2" */
                variants={{
                    folded: {
                        rotate: 0,
                    },
                    open: {
                        rotate: 180,
                    },
                }}
            />
        </div>
    );
};

FoldableAdmonitionTitle.displayName = "FoldableAdmonitionTitle";

export default FoldableAdmonitionTitle;
