import React, { HTMLProps, type ReactNode } from "react";
import {
    WithColorKey,
} from "../embeddable_component_types/color_key";
import { ulPropsSchema } from "./ul_props_schema";
import { cn } from "@/utils/cn";

export interface UlProps extends HTMLProps<HTMLSpanElement>, WithColorKey {
    /// A valid fluster color variable.
    color: string;
    children: ReactNode;
}

export const Ul = ({ children, ...props }: UlProps): ReactNode => {
    const parsedClasses = ulPropsSchema.parse(props)
    return (
        <span
            {...props}
            style={{
                paddingLeft: "1ch",
                paddingRight: "1ch",
                ...props.style,
            }}
            className={cn("[&_*]:text-inherit underline", ...parsedClasses)}
        >
            {children}
        </span>
    );
};

Ul.displayName = "Ul";
