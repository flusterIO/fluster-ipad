"use client";
import React, {
    type RefObject,
    type ComponentPropsWithoutRef,
    type ReactNode,
} from "react";
import { BackgroundGradient } from "./background_gradient";
import { cn } from "#/core/utils/cn";

interface BackgroundGradientCardProps extends ComponentPropsWithoutRef<
    typeof BackgroundGradient
> {
    children: ReactNode;
    backgroundRef: RefObject<HTMLDivElement>;
}

export const BackgroundGradientCard = ({
    children,
    className,
    backgroundRef,
    ...props
}: BackgroundGradientCardProps) => {
    return (
        <BackgroundGradient
            className={cn("rounded-3xl p-4 sm:p-6 bg-card", className)}
            ref={backgroundRef}
            {...props}
        >
            {children}
        </BackgroundGradient>
    );
};

BackgroundGradientCard.displayName = "BackgroundGradientCard";
