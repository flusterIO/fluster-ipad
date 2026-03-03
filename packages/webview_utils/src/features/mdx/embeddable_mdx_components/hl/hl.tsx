import React, { HTMLProps, type ReactNode } from "react";
import {
    WithColorKey,
} from "../embeddable_component_types/color_key";
import { hlPropsSchema } from "./hl_props_schema";
import { cn } from "@/utils/cn";

export interface HlProps extends HTMLProps<HTMLSpanElement>, WithColorKey {
    /// A valid fluster color variable.
    color: string;
    children: ReactNode;
}

export const Hl = ({ children, ...props }: HlProps): ReactNode => {
    const colorClasses = hlPropsSchema.parse(props)
    return (
        <span
            {...props}
            style={{
                paddingLeft: "0.2rem",
                paddingRight: "0.2rem",
                borderRadius: "4px",
                ...props.style,
            }}
            className={cn("[&_*]:text-inherit", colorClasses)}
        >
            {children}
        </span>
    );
};

Hl.displayName = "Hl";
