import { useIsomorphicLayoutEffect } from "framer-motion";
import { type ReactNode } from "react";
import { useDarkMode } from "./use_dark_mode";

export const DarkModeObserver = (): ReactNode => {
    const darkMode = useDarkMode();
    useIsomorphicLayoutEffect(() => {
        document.body.classList[darkMode ? "add" : "remove"]("dark");
    }, [darkMode]);
    return null;
};

DarkModeObserver.displayName = "DarkModeObserver";
