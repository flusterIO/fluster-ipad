import React, { HTMLProps, type ReactNode } from "react";
import {
    ColorClassMap,
    getColorKey,
    WithColorKey,
} from "./embeddable_component_types/color_key";
import clsx from "clsx";

export const ComponentName = "Hl";

export interface HlProps extends HTMLProps<HTMLSpanElement>, WithColorKey {
    /// A valid fluster color variable.
    color: string;
    children: ReactNode;
}

export const Hl = ({ children, ...props }: HlProps): ReactNode => {
    const classes: ColorClassMap = {
        primary: "bg-primary text-primary-foreground",
        secondary: "bg-secondary text-secondary-foreground",
        error: "bg-destructive text-destructive-foreground",
        muted: "bg-muted/50 text-muted-foreground",
    };
    const colorKey = getColorKey(props, "primary");
    return (
        <span
            {...props}
            style={{
                paddingLeft: "0.2rem",
                paddingRight: "0.2rem",
                borderRadius: "4px",
                ...props.style,
            }}
            className={clsx("[&_*]:text-inherit", classes[colorKey])}
        >
            {children}
        </span>
    );
};

Hl.displayName = "Hl";
