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

export {
    CodeEditorTheme
}


export type CodeEditorImplementation = "bib-editor" | "mdx-editor" | "mdx-viewer" | "development"
