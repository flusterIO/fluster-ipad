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
    [CodeEditorTheme.Dracula]: () => dracula,
    [CodeEditorTheme.MaterialDark]: () => materialDark,
    [CodeEditorTheme.MaterialLight]: () => materialLight,
    [CodeEditorTheme.Aura]: () => aura,
    [CodeEditorTheme.GithubDark]: () => githubDark,
    [CodeEditorTheme.GithubLight]: () => githubLight,
    [CodeEditorTheme.TokyoNight]: () => tokyoNight,
    [CodeEditorTheme.TokyoNightDay]: () => tokyoNightDay,
    [CodeEditorTheme.TokyoNightStorm]: () => tokyoNightStorm,
    [CodeEditorTheme.SolarizedLight]: () => solarizedLight,
    [CodeEditorTheme.SolarizedDark]: () => solarizedDark,
    [CodeEditorTheme.XcodeDark]: () => xcodeDark,
    [CodeEditorTheme.XcodeLight]: () => xcodeLight,
};
