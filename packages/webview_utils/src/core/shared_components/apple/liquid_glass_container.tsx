"use client";
import React, { HTMLProps, useState } from "react";

import { ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

interface LiquidGlassCardProps extends Omit<HTMLProps<HTMLDivElement>, "onClick"> {
    children?: React.ReactNode;
    className?: string;
    draggable?: boolean;
    expandable?: boolean;
    width?: string;
    height?: string;
    blurIntensity?: "sm" | "md" | "lg" | "xl";
    shadowIntensity?: "none" | "xs" | "sm" | "md" | "lg" | "xl";
    borderRadius?: string;
    glowIntensity?: "none" | "xs" | "sm" | "md" | "lg" | "xl";
}

export const LiquidGlassCard = ({
    children = null,
    className = "",
    draggable = true,
    expandable = false,
    width,
    height,
    blurIntensity = "xl",
    borderRadius = "32px",
    glowIntensity = "sm",
    shadowIntensity = "md",
    ...props
}: LiquidGlassCardProps) => {

    const blurClasses = {
        sm: "backdrop-blur-xs",
        md: "backdrop-blur-md",
        lg: "backdrop-blur-lg",
        xl: "backdrop-blur-xl",
    };

    const shadowStyles = {
        none: "inset 0 0 0 0 rgba(255, 255, 255, 0)",
        xs: "inset 1px 1px 1px 0 rgba(255, 255, 255, 0.3), inset -1px -1px 1px 0 rgba(255, 255, 255, 0.3)",
        sm: "inset 2px 2px 2px 0 rgba(255, 255, 255, 0.35), inset -2px -2px 2px 0 rgba(255, 255, 255, 0.35)",
        md: "inset 3px 3px 3px 0 rgba(255, 255, 255, 0.45), inset -3px -3px 3px 0 rgba(255, 255, 255, 0.45)",
        lg: "inset 4px 4px 4px 0 rgba(255, 255, 255, 0.5), inset -4px -4px 4px 0 rgba(255, 255, 255, 0.5)",
        xl: "inset 6px 6px 6px 0 rgba(255, 255, 255, 0.55), inset -6px -6px 6px 0 rgba(255, 255, 255, 0.55)",
        "2xl":
            "inset 8px 8px 8px 0 rgba(255, 255, 255, 0.6), inset -8px -8px 8px 0 rgba(255, 255, 255, 0.6)",
    };

    const glowStyles = {
        none: "0 4px 4px rgba(0, 0, 0, 0.05), 0 0 12px rgba(0, 0, 0, 0.05)",
        xs: "0 4px 4px rgba(0, 0, 0, 0.15), 0 0 12px rgba(0, 0, 0, 0.08), 0 0 16px rgba(255, 255, 255, 0.05)",
        sm: "0 4px 4px rgba(0, 0, 0, 0.15), 0 0 12px rgba(0, 0, 0, 0.08), 0 0 24px rgba(255, 255, 255, 0.1)",
        md: "0 4px 4px rgba(0, 0, 0, 0.15), 0 0 12px rgba(0, 0, 0, 0.08), 0 0 32px rgba(255, 255, 255, 0.15)",
        lg: "0 4px 4px rgba(0, 0, 0, 0.15), 0 0 12px rgba(0, 0, 0, 0.08), 0 0 40px rgba(255, 255, 255, 0.2)",
        xl: "0 4px 4px rgba(0, 0, 0, 0.15), 0 0 12px rgba(0, 0, 0, 0.08), 0 0 48px rgba(255, 255, 255, 0.25)",
        "2xl":
            "0 4px 4px rgba(0, 0, 0, 0.15), 0 0 12px rgba(0, 0, 0, 0.08), 0 0 60px rgba(255, 255, 255, 0.3)",
    };

    return (
        <>
            <svg className="hidden">
                <defs>
                    <filter
                        id="glass-blur"
                        x="0"
                        y="0"
                        width="100%"
                        height="100%"
                        filterUnits="objectBoundingBox"
                    >
                        <feTurbulence
                            type="fractalNoise"
                            baseFrequency="0.003 0.007"
                            numOctaves="1"
                            result="turbulence"
                        />
                        <feDisplacementMap
                            in="SourceGraphic"
                            in2="turbulence"
                            scale="200"
                            xChannelSelector="R"
                            yChannelSelector="G"
                        />
                    </filter>
                </defs>
            </svg>
            <div
                className={cn(
                    `relative ${draggable ? "cursor-grab active:cursor-grabbing" : ""} ${expandable ? "cursor-pointer" : ""}`,
                    className,
                )}
                style={{
                    borderRadius,
                    ...(width && !expandable && { width }),
                    ...(height && !expandable && { height }),
                }}
                {...props}
            >
                {/* Bend Layer (Backdrop blur with distortion) */}
                <div
                    className={`absolute inset-0 ${blurClasses[blurIntensity]} z-0`}
                    style={{
                        borderRadius,
                        filter: "url(#glass-blur)",
                    }}
                />

                {/* Face Layer (Main shadow and glow) */}
                <div
                    className="absolute inset-0 z-10"
                    style={{
                        borderRadius,
                        boxShadow: glowStyles[glowIntensity],
                    }}
                />

                {/* Edge Layer (Inner highlights) */}
                <div
                    className="absolute inset-0 z-20"
                    style={{
                        borderRadius,
                        boxShadow: shadowStyles[shadowIntensity],
                    }}
                />

                {/* Content */}
                {children}
            </div>
        </>
    );
};
