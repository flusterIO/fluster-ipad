import React, { type ReactNode } from "react";
import { SquareLoader } from "react-spinners";

export const LoadingIndicator = (): ReactNode => {
    return <SquareLoader color="hsl(var(--color-primary))" />;
};

LoadingIndicator.displayName = "LoadingIndicator";
