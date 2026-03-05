import { defaultKeymap } from "@codemirror/commands";
import { vscodeKeymap } from "@replit/codemirror-vscode-keymap";
import { KeyBinding } from "@codemirror/view";
import { CodeEditorBaseKeymap } from "@/code_gen/typeshare/fluster_core_utilities";

export const codeEditorBaseKeymapMap: Record<
    CodeEditorBaseKeymap,
    () => readonly KeyBinding[]
> = {
    [CodeEditorBaseKeymap.Default]: () => defaultKeymap,
    [CodeEditorBaseKeymap.VsCode]: () => vscodeKeymap,
};
