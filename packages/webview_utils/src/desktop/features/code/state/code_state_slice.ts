import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { BundledLanguage } from "shiki";
import { CodeState, JupyterConfigState } from "./code_state";
import { initialCodeState } from "./initial_code_state";
import { BundledFlusterTheme } from "../data/bundled_themes";

const slice = createSlice({
    name: "code",
    initialState: initialCodeState,
    reducers: {
        setEditorKeymap(state, action: PayloadAction<CodeState["keymap"]>) {
            return {
                ...state,
                keymap: action.payload,
            };
        },
        setDefaultLanguage(state, action: PayloadAction<BundledLanguage>) {
            return {
                ...state,
                defaultLanguage: action.payload,
            };
        },
        setCodeTheme(
            state,
            action: PayloadAction<{
                themeMode: "light" | "dark";
                value: BundledFlusterTheme;
            }>
        ) {
            return {
                ...state,
                theme: {
                    ...state.theme,
                    [action.payload.themeMode]: action.payload.value,
                },
            };
        },
        setJupyterState(state, action: PayloadAction<Partial<JupyterConfigState>>) {
            return {
                ...state,
                jupyter: {
                    ...state.jupyter,
                    ...action.payload,
                },
            };
        },
        setPreviewDebounce(state, action: PayloadAction<number>) {
            return {
                ...state,
                previewDebounce: action.payload,
            };
        },
    },
});

export const {
    setCodeTheme,
    setDefaultLanguage,
    setEditorKeymap,
    setJupyterState,
    setPreviewDebounce,
} = slice.actions;

export default slice.reducer;
