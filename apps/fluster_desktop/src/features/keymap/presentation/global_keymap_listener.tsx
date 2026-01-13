import React, { type ReactNode } from "react";
import { useGlobalKeymap } from "../state/hooks/use_global_keymap";

export const GlobalKeymapListener = (): ReactNode => {
    useGlobalKeymap();
    return null;
};

GlobalKeymapListener.displayName = "GlobalKeymapListener";
