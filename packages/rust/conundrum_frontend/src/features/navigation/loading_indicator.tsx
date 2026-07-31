import React, { type ReactNode } from "react";
import { SquareLoader, PulseLoader } from "react-spinners";

export const LoadingIndicator = (): ReactNode => {
    return <PulseLoader color="hsl(var(--primary))" />;
};

LoadingIndicator.displayName = "LoadingIndicator";
