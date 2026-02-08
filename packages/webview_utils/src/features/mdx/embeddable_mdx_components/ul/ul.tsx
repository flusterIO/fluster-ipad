import React, { HTMLProps, type ReactNode } from "react";
import {
    ColorCssMap,
    getColorKey,
    WithColorKey,
} from "../embeddable_component_types/color_key";

export interface UlProps extends HTMLProps<HTMLSpanElement>, WithColorKey {
    /// A valid fluster color variable.
    color: string;
    children: ReactNode;
}

export const Ul = ({ children, ...props }: UlProps): ReactNode => {
    const styleMap: ColorCssMap = {
        primary: {
            textDecorationColor: "hsl(var(--primary))",
        },
        secondary: {
            textDecorationColor: "hsl(var(--secondary))",
        },
        error: {
            textDecorationColor: "hsl(var(--destructive))",
        },
        muted: {
            textDecorationColor: "hsl(var(--muted-foreground))",
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
            className="[&_*]:text-inherit underline"
        >
            {children}
        </span>
    );
};

Ul.displayName = "Ul";
