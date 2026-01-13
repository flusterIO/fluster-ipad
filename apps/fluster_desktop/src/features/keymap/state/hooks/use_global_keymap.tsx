import { keymapActionMap } from "#/keymap/data/keymap_action_map";
import { AppState } from "@/state/initial_state";
import { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { KeymapId } from "../keymap_state";

export const useGlobalKeymap = () => {
    const keymapState = useSelector((state: AppState) => state.keymap);
    const dispatch = useDispatch();

    const handleKeyDown = (e: KeyboardEvent): void => {
        for (const k in keymapState) {
            const item = keymapState[k as keyof typeof keymapState];
            if (
                item.shift === e.shiftKey &&
                item.meta === e.metaKey &&
                item.ctrl === e.ctrlKey &&
                item.keyCode === e.keyCode
            ) {
                return keymapActionMap[k as KeymapId](dispatch);
            }
        }
    };
    useEffect(() => {
        window.addEventListener("keydown", handleKeyDown);
        return () => window.removeEventListener("keydown", handleKeyDown);
    }, []);
};
