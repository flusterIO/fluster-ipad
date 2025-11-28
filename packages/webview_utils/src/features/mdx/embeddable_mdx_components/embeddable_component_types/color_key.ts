import { CSSProperties } from "react";

const colorKeys = ["primary", "secondary", "muted", "error"] as const;
export type ColorKey = (typeof colorKeys)[number];

export type WithColorKey = {
    [key in ColorKey]?: boolean;
};

export type ColorClassMap = {
    [key in ColorKey]: string;
};

export type ColorCssMap = {
    [key in ColorKey]: CSSProperties;
};

export const getColorKey = (
    props: WithColorKey,
    defaultKey: ColorKey = "primary",
): ColorKey => {
    for (const k of colorKeys) {
        if (props[k]) {
            return k;
        }
    }
    return defaultKey;
};
