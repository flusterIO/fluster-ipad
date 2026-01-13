import { Dispatch } from "@reduxjs/toolkit";
import { KeymapId } from "../state/keymap_state";
import { setCommandPaleteOpen } from "#/command_palette/state/command_palette_state_slice";

export const keymapActionMap: {
    [K in KeymapId]: (dispatch: Dispatch) => void;
} = {
    toggleCommandPalette: (d) => d(setCommandPaleteOpen("toggle")),
};
