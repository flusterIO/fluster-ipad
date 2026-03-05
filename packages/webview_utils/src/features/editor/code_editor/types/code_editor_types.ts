export { CodeEditorBaseKeymap } from "@/code_gen/typeshare/fluster_core_utilities";
import { CodeEditorTheme } from "@/code_gen/typeshare/fluster_core_utilities"



export const stringToCodeEditorTheme = (val: string): CodeEditorTheme => {
    if (Object.values(val).includes(val)) {
        return val as CodeEditorTheme
    } else {
        return CodeEditorTheme.Dracula
    }
};

export enum CodeEditorLanguage {
    markdown,
    bibtex,
}

declare global {
    interface WindowEventMap {
        "set-code-theme": CustomEvent<string>;
        "set-editor-keymap": CustomEvent<string>;
        "set-initial-editor-content": CustomEvent<string>;
    }
}

export {
    CodeEditorTheme
}
