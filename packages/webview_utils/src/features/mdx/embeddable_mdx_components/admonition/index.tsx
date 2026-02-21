import React, { FC, useEffect, useEffectEvent, useMemo, useState, type ReactNode } from "react";
import { motion } from "framer-motion";
import AdmonitionTitle from "./admonition_title";
import FoldableAdmonitionTitle from "./foldable_admonition_title";
import { AdmonitionTitleProps, AdmonitionVariant } from "./types";
import { getPositionableClasses } from "#/mdx/mdx_utils/get_positionable_classes";
import { cn } from "@/utils/cn";
import { PositionableProps } from "../embeddable_component_types/positionable";
import { inlineMdxClasses } from "../../components/inline_mdx_classes";

export interface AdmonitionProps
    extends Omit<AdmonitionTitleProps, "type">,
    PositionableProps {
    type?: AdmonitionVariant;
    /// Start the admonition in a folded state.
    folded?: boolean;
    /// Whether or not to make the admonition foldable.
    foldable?: boolean;
    children: ReactNode;
    /// InlineMdxContent are passed in automatically in the component map.
    InlineMdxContent: FC<{ mdx: string }>;
    classes?: {
        container?: string
        body?: string
    }
}

export const Admonition = ({
    folded,
    children,
    type = "primary",
    foldable,
    InlineMdxContent,
    title: _title,
    classes = {},
    ...props
}: AdmonitionProps): ReactNode => {
    const [open, setOpen] = useState(foldable ? !folded : true);
    // sure to enforce the type safety for all string components before use as there's no typesafety while the mdx is rendered in live preview.
    const title = typeof _title === "string" ? _title : "";

    const handleOpen = useEffectEvent((newOpen: boolean) => setOpen(newOpen))

    useEffect(() => {
        if (!foldable) {
            handleOpen(true);
        }
    }, [foldable]);

    const titleComponent = useMemo(() => {
        return InlineMdxContent ? <InlineMdxContent mdx={title} /> : <div className={inlineMdxClasses}>{title}</div>;
    }, [title, InlineMdxContent]);

    return (
        <motion.div
            initial={folded && foldable ? "folded" : "open"}
            animate={open ? "open" : "folded"}
            className={cn("my-4 overflow-hidden", getPositionableClasses(props), classes.container)}
        >
            {foldable ? (
                <FoldableAdmonitionTitle
                    open={open}
                    setOpen={setOpen}
                    title={title}
                    type={type}
                >
                    {titleComponent}
                </FoldableAdmonitionTitle>
            ) : (
                <AdmonitionTitle title={title} type={type}>
                    {titleComponent}
                </AdmonitionTitle>
            )}
            <motion.div
                className={"rounded-bl rounded-br border-l border-b border-r bg-card text-card-foreground relative"}
                variants={{
                    folded: {
                        height: 0,
                        opacity: 0,
                    },
                    open: {
                        height: "fit-content",
                        opacity: 1,
                    },
                }}
                transition={{
                    bounce: 0,
                }}
            >
                <div className={cn("p-4 first:mt-0 last:mb-0 admonitionBody inline-mdx", classes.body)}>{children}</div>
            </motion.div>
        </motion.div>
    );
};

Admonition.displayName = "Admonition";
