import { useCommandPaletteDispatch } from "#/command_palette/command_palette_provider";
import { useBackendPinger } from "#/database/backend_pinger";
import { useGlobalKeyboardListener } from "#/keyboard/use_global_keyboard_listener";
import { useGlobalTimers } from "@/hooks/use_state_based_timers";
import React, { useEffect, type ReactNode } from "react";

export const GlobalListeners = (): ReactNode => {
    useGlobalKeyboardListener();
    useBackendPinger();
    useGlobalTimers();

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

GlobalListeners.displayName = "GlobalListeners";
