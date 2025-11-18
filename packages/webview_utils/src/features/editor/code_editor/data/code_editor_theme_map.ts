import { Extension } from "@codemirror/state";
import { CodeEditorTheme } from "../types/code_editor_types";
import { materialLight } from "@ddietr/codemirror-themes/material-light";
import { materialDark } from "@ddietr/codemirror-themes/material-dark";
import { solarizedLight } from "@ddietr/codemirror-themes/solarized-light";
import { solarizedDark } from "@ddietr/codemirror-themes/solarized-dark";
import { dracula } from "@ddietr/codemirror-themes/dracula";
import { githubLight } from "@ddietr/codemirror-themes/github-light";
import { githubDark } from "@ddietr/codemirror-themes/github-dark";
import { aura } from "@ddietr/codemirror-themes/aura";
import { tokyoNight } from "@ddietr/codemirror-themes/tokyo-night";
import { tokyoNightStorm } from "@ddietr/codemirror-themes/tokyo-night-storm";
import { tokyoNightDay } from "@ddietr/codemirror-themes/tokyo-night-day";
import { xcodeLight, xcodeDark } from '@uiw/codemirror-theme-xcode';

export const codeEditorThemeMap: Record<CodeEditorTheme, () => Extension> = {
    [CodeEditorTheme.dracula]: () => dracula,
    [CodeEditorTheme.materialDark]: () => materialDark,
    [CodeEditorTheme.materialLight]: () => materialLight,
    [CodeEditorTheme.aura]: () => aura,
    [CodeEditorTheme.githubDark]: () => githubDark,
    [CodeEditorTheme.githubLight]: () => githubLight,
    [CodeEditorTheme.tokyoNight]: () => tokyoNight,
    [CodeEditorTheme.tokyoNightDay]: () => tokyoNightDay,
    [CodeEditorTheme.tokyoNightStorm]: () => tokyoNightStorm,
    [CodeEditorTheme.solarizedLight]: () => solarizedLight,
    [CodeEditorTheme.solarizedDark]: () => solarizedDark,
    [CodeEditorTheme.xcodeDark]: () => xcodeDark,
    [CodeEditorTheme.xcodeLight]: () => xcodeLight,
};
