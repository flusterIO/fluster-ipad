import React, { HTMLProps, type ReactNode } from "react";
import {
    ColorCssMap,
    getColorKey,
    WithColorKey,
} from "./embeddable_component_types/color_key";

export interface HlProps extends HTMLProps<HTMLSpanElement>, WithColorKey {
    /// A valid fluster color variable.
    color: string;
    children: ReactNode;
}

export const Hl = ({ children, ...props }: HlProps): ReactNode => {
    const styleMap: ColorCssMap = {
        primary: {
            backgroundColor: "hsl(var(--primary))",
            color: "hsl(var(--primary-foreground))",
        },
        secondary: {
            backgroundColor: "hsl(var(--secondary))",
            color: "hsl(var(--secondary-foreground))",
        },
        error: {
            backgroundColor: "hsl(var(--destructive))",
            color: "hsl(var(--destructive-foreground))",
        },
        muted: {
            backgroundColor: "hsl(var(--muted)/40)",
            color: "hsl(var(--muted-foreground))",
        },
    };
    const colorKey = getColorKey(props, "primary");
    return (
        <span
            {...props}
            style={{
                paddingLeft: "0.2rem",
                paddingRight: "0.2rem",
                borderRadius: "4px",
                ...styleMap[colorKey],
                ...props.style,
            }}
            className="[&_*]:text-inherit"
        >
            {children}
        </span>
    );
};

Hl.displayName = "Hl";
