import { KeyBinding } from "@codemirror/view";
import { useCodeEditorContext } from "../state/code_editor_provider";
import { useEffect, useRef } from "react";
import { EditorClient } from "../data/editor_client";


// export const useEditorKeymap = (): readonly KeyBinding[] => {
//     const { note_id } = useCodeEditorContext();
//     const noteId = useRef<string | null>(null);
//     useEffect(() => {
//
//         // Hack to make this available in the function.
//         noteId.current = note_id ?? null
//     }, [note_id])
//     return ;
// }
