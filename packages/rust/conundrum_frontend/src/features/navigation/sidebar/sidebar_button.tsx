import { type HTMLProps } from "@base-ui/react";
import React, { type FC, type ReactNode } from "react";
import { Link } from "react-router";
import { motion } from "framer-motion";

export type PermanentSidebarButtonProps = {
    icon: FC<{ className?: string }>;
    /// Set to true for non-conundrum links
    isNonLocal?: boolean;
    active: boolean;
} & (
        | {
            onClick?: () => void;
            href?: undefined;
            id: string;
        }
        | {
            onClick?: undefined;
            href?: string;
        }
    );

export const PermanentSidebarButton = ({
    onClick,
    isNonLocal,
    href,
    icon: Icon,
    active,
}: PermanentSidebarButtonProps): ReactNode => {
    const props: HTMLProps<HTMLAnchorElement> = {
        ...(href ? { href } : { role: "button", onClick }),
    };
    if (href && !isNonLocal) {
        if (active) {
            return (
                <Link to={href} className={"grid place-items-center"}>
                    <motion.div
                        initial={{
                            backgroundColor: "transparent",
                        }}
                        animate={{ backgroundColor: "hsl(var(--primary))" }}
                        className="rounded p-1"
                    >
                        <Icon className="w-6 h-6" />
                    </motion.div>
                </Link>
            );
        }
        return (
            <Link to={href} className="grid place-items-center">
                <div className={"p-1"}>
                    <Icon className="w-6 h-6" />
                </div>
            </Link>
        );
    }
    return (
        <a {...props} className="grid place-items-center">
            <Icon className="w-6 h-6" />
        </a>
    );
};

PermanentSidebarButton.displayName = "PermanentSidebarButton";
