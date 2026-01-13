import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { KeymapItem } from "./keymap_state";
import { initialKeymapState } from "./initial_keymap_state";

const slice = createSlice({
    name: "keymap",
    initialState: initialKeymapState,
    reducers: {
        setKeymapItem(
            state,
            action: PayloadAction<{
                KeymapId: KeymapItem;
            }>,
        ) {
            state = {
                ...state,
                ...action.payload,
            };
        },
    },
});

export const { setKeymapItem } = slice.actions;

export default slice.reducer;
