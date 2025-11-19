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
import { stringToCodeEditorTheme } from "../types/code_editor_types";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { lineNumbersRelative } from "@uiw/codemirror-extensions-line-numbers-relative";
import { sendToSwift, SwiftHandler } from "@/utils/bridge/send_to_swift";
import { useLocalStorage } from "@/state/hooks/use_local_storage";
import { LoadingComponent } from "@/shared_components/loading_component";

interface CodeEditorProps {
    language?: Extension;
    initialValue: string;
}

export const CodeEditorInner = ({
    language = markdown(),
    initialValue,
}: CodeEditorProps): ReactNode => {
    const haveRendered = useRef(false);
    const state = useCodeEditorContext();
    const dispatch = useCodeEditorDispatch();
    const timer = useRef<NodeJS.Timeout | null>(null);
    useEffect(() => {
        if (haveRendered.current) {
            haveRendered.current = false;
            document.getElementById("code-editor-container")!.replaceChildren();
        }
        let extensions: Extension[] = [];
        if (state.vimMode) {
            extensions.push(vim());
            extensions.push(lineNumbersRelative);
        }
        extensions = [
            ...extensions,
            keymap.of(codeEditorBaseKeymapMap[state.baseKeymap]()),
            EditorState.allowMultipleSelections.of(true),
            EditorView.lineWrapping,
            language,
            codeEditorThemeMap[state.theme](),
            EditorView.updateListener.of((v: ViewUpdate) => {
                if (!state.haveSetInitialValue) {
                    return;
                }
                if (v.docChanged) {
                    const payload = v.state.doc.toString();
                    if (timer.current) {
                        clearTimeout(timer.current);
                    }
                    timer.current = setTimeout(() => {
                        sendToSwift(SwiftHandler.editorUpdate, payload);
                    }, 500);
                    dispatch({
                        type: "setValue",
                        payload,
                    });
                }
            }),
        ];
        const startState = EditorState.create({
            doc: initialValue,
            extensions,
        });

        dispatch({
            type: "setInitialEditorValue",
            payload: initialValue,
        });

        const _view = new EditorView({
            state: startState,
            parent: document.getElementById("code-editor-container")!,
        });
        _view.focus();
        haveRendered.current = true;
        /* eslint-disable-next-line  -- Don't want to run it on the other value change. */
    }, [state.baseKeymap, state.theme, state.haveSetInitialValue]);
    useEventListener("set-editor-theme", (e) => {
        window.alert(`Set Theme: ${e.detail}`);
        dispatch({
            type: "setTheme",
            payload: stringToCodeEditorTheme(e.detail),
        });
    });
    useEventListener("set-editor-keymap", (e) => {
        window.alert(`Set keymap: ${e.detail}`);
        dispatch({
            type: "setVimMode",
            payload: e.detail === "vim",
        });
    });

    useEventListener("set-initial-editor-content", (e) => {
        window.alert(`Set initial value: ${e.detail}`);
        dispatch({
            type: "setInitialEditorValue",
            payload: e.detail,
        });
    });
    useEffect(() => {
        return () => window.localStorage.removeItem("editor-initial-value");
    }, []);
    return <div className="h-full w-full" id="code-editor-container" />;
};

export const CodeEditor = (): ReactNode => {
    const [value] = useLocalStorage("editor-initial-value");
    return typeof value === "string" ? (
        <CodeEditorInner initialValue={value} />
    ) : (
        <div className="w-full h-full flex flex-col justify-center items-center">
            <LoadingComponent />
        </div>
    );
};

CodeEditor.displayName = "CodeEditor";
