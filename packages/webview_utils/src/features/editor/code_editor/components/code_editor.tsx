import React, { useEffect, useRef, type ReactNode } from "react";
import { EditorState, Extension } from "@codemirror/state";
import { EditorView, keymap, ViewUpdate } from "@codemirror/view";
import { markdown } from "@codemirror/lang-markdown";
import { vim } from "@replit/codemirror-vim";
import {
    useCodeEditorContext,
    useCodeEditorDispatch,
} from "../state/code_editor_provider";
import { codeEditorBaseKeymapMap } from "../data/code_editor_base_keymap_map";
import { codeEditorThemeMap } from "../data/code_editor_theme_map";

export const CodeEditor = (): ReactNode => {
    const haveRendered = useRef(false);
    const state = useCodeEditorContext();
    const dispatch = useCodeEditorDispatch();
    useEffect(() => {
        if (haveRendered.current) {
            haveRendered.current = false;
            document.getElementById("code-editor-container")!.replaceChildren();
        }
        let extensions: Extension[] = [];
        if (state.vimMode) {
            extensions.push(vim());
        }
        extensions = [
            ...extensions,
            keymap.of(codeEditorBaseKeymapMap[state.baseKeymap]()),
            EditorState.allowMultipleSelections.of(true),
            EditorView.lineWrapping,
            markdown(),
            codeEditorThemeMap[state.theme](),
            EditorView.updateListener.of((v: ViewUpdate) => {
                if (v.docChanged) {
                    dispatch({
                        type: "setValue",
                        payload: v.state.doc.toString(),
                    });
                }
            }),
        ];
        const startState = EditorState.create({
            doc: state.value,
            extensions,
        });

        const _view = new EditorView({
            state: startState,
            parent: document.getElementById("code-editor-container")!,
        });
        _view.focus();
        haveRendered.current = true;
        /* eslint-disable-next-line  -- Don't want to run it on the other value change. */
    }, [state.baseKeymap, state.theme]);
    return <div className="h-full w-full" id="code-editor-container" />;
};

CodeEditor.displayName = "CodeEditor";
