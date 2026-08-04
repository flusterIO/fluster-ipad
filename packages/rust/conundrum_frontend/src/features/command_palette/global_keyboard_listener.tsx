import { useEffect, type ReactNode } from "react";
import { useCommandPaletteDispatch } from "./command_palette_provider";

export const GlobalKeyboardListener = (): ReactNode => {
    const commandPaletteDispatch = useCommandPaletteDispatch();
    const handleGlobalKeyDown = (e: KeyboardEvent): void => {
        if (e.shiftKey && e.metaKey && e.key === "p") {
            commandPaletteDispatch({
                type: "openCommandPalette",
            });
        }
    };
    useEffect(() => {
        window.addEventListener("keydown", handleGlobalKeyDown);
        return () => {
            window.removeEventListener("keydown", handleGlobalKeyDown);
        };
    }, []);
    return null;
};

GlobalKeyboardListener.displayName = "GlobalKeyboardListener";
