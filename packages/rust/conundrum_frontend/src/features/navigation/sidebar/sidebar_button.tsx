import { type HTMLProps } from "@base-ui/react";
import React, { type FC, type ReactNode } from "react";
import { Link } from "react-router";

export type PermanentSidebarButtonProps = {
    icon: FC<{ className?: string }>;
    /// Set to true for non-conundrum links
    isNonLocal?: boolean;
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
}: PermanentSidebarButtonProps): ReactNode => {
    const props: HTMLProps<HTMLAnchorElement> = {
        ...(href ? { href } : { role: "button", onClick }),
    };
    if (href && !isNonLocal) {
        return (
            <Link to={href} className="grid place-items-center">
                <Icon className="w-6 h-6" />
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
