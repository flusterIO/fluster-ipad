import { KeymapItem, KeymapState } from "./keymap_state";

export const initialKeymapState: KeymapState = {
    toggleCommandPalette: {
        ctrl: false,
        meta: true,
        shift: false,
        keyCode: 80,
    },
};
