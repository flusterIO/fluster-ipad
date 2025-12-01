import { CSSProperties, useState } from "react";
import { useEventListener } from "./use_event_listener";

export interface ScreenDimensions {
    width: number | null;
    height: number | null;
}

export const useScreenDimensions = (
    screenDimensionCalculator?: (dims: ScreenDimensions) => CSSProperties,
): CSSProperties | null => {
    const [calculatedDimensions, setCalculatedDimensions] =
        useState<CSSProperties | null>(null);

    useEventListener("set-screen-size", (e) => {
        if (!screenDimensionCalculator) {
            return;
        }
        setCalculatedDimensions(
            screenDimensionCalculator({
                width: e.detail.width,
                height: e.detail.height,
            }),
        );
    });

    return calculatedDimensions;
};
