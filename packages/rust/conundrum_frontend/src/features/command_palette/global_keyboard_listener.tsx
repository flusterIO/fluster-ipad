import { setCommandPaletteOpen } from "#/navigation/state/navigation_slice";
import { useEffect, type ReactNode } from "react";
import { useDispatch } from "react-redux";
import { useCommandPaletteDispatch } from "./command_palette_provider";

export const GlobalKeyboardListener = (): ReactNode => {
    const dispatch = useDispatch();
    const commandPaletteDispatch = useCommandPaletteDispatch();
    const handleGlobalKeyDown = (e: KeyboardEvent): void => {
        console.log("e: ", e);
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
