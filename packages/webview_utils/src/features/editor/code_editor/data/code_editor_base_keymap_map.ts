import { CodeEditorBaseKeymap } from "../types/code_editor_types";
import { defaultKeymap } from "@codemirror/commands";
import { vscodeKeymap } from "@replit/codemirror-vscode-keymap";
import { KeyBinding } from "@codemirror/view";

export const codeEditorBaseKeymapMap: Record<
    CodeEditorBaseKeymap,
    () => readonly KeyBinding[]
> = {
    [CodeEditorBaseKeymap.default]: () => defaultKeymap,
    [CodeEditorBaseKeymap.vscode]: () => vscodeKeymap,
};
