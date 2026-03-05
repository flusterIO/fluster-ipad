import { KeyBinding } from "@codemirror/view";
import { useCodeEditorContext } from "../state/code_editor_provider";
import { useEffect, useRef } from "react";
import { EditorClient } from "../data/editor_client";


export const useEditorKeymap = (): readonly KeyBinding[] => {
    const { note_id } = useCodeEditorContext();
    const noteId = useRef<string | null>(null);
    useEffect(() => {
        // Hack to make this available in the function.
        noteId.current = note_id
    }, [note_id])
    return [{
        key: "Mod-S",
        run: (view) => {
            const content = view.state.doc.toString()
            if (noteId.current) {
                EditorClient.sendManualSaveRequest({
                    note_id: noteId.current,
                    current_note_content: content
                })
                return true;
            } else {
                return false
            }
        },
        preventDefault: true
    },
        // {
        // TODO: Use this to allow the user to refresh the previous *without* refreshing the editor.
        //     key: "Ctrl-R", // Example: Capture Ctrl-R
        //     run: (view) => {
        //         console.log("Ctrl-R was pressed!");
        //         // Perform your custom action here
        //         return true; // Return true to indicate the event was handled
        //     },
        //     // Prevent default browser behavior (like refreshing the page for Ctrl-R)
        //     preventDefault: true
        // }
        /* { */
        /*     key: "any", // A specific "any" key can be used for every keypress */
        /*     run: (view, event) => { */
        /*         console.log("A key was pressed:", event.key); */
        /*         // Returning false (or nothing) lets other keymaps or default behavior continue */
        /*         return false; */
        /*     } */
        /* } */
    ];
}
