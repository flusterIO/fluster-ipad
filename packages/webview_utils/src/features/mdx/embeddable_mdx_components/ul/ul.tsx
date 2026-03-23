import React, { type HTMLProps, type ReactNode } from "react";
import { type ULProps, ulPropsSchema } from "./ul_props_schema";
import { cn } from "@/utils/cn";

interface UlPropsInternal extends ULProps, Omit<HTMLProps<HTMLSpanElement>, "color"> {
    children: ReactNode;
}

export const Ul = ({ children, ...props }: UlPropsInternal): ReactNode => {
    const parsedClasses = ulPropsSchema.parse(props)
    return (
        <span
            {...props}
            style={{
                /* paddingLeft: "1ch", */
                /* paddingRight: "1ch", */
                ...props.style,
            }}
            className={cn("[&_*]:text-inherit underline", ...parsedClasses)}
        >
            {children}
        </span>
    );
};

Ul.displayName = "Ul";
