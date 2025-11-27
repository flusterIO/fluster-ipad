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
import { lineNumbersRelative } from "@uiw/codemirror-extensions-line-numbers-relative";
import { sendToSwift, SwiftHandler } from "@/utils/bridge/send_to_swift";
import { LoadingComponent } from "@/shared_components/loading_component";
import { useLocalStorage } from "@/state/hooks/use_local_storage";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { setWindowBridgeFunctions } from "../types/swift_events/swift_events";

interface CodeEditorProps {
    language?: Extension;
    initialValue: string;
    updateHandler?: SwiftHandler;
}

setWindowBridgeFunctions();

export const CodeEditorInner = ({
    language = markdown(),
    initialValue,
    updateHandler = SwiftHandler.editorUpdate,
}: CodeEditorProps): ReactNode => {
    const haveRendered = useRef(false);
    const state = useCodeEditorContext();
    const dispatch = useCodeEditorDispatch();
    const timer = useRef<NodeJS.Timeout | null>(null);
    const view = useRef<EditorView | null>(null);
    useEffect(() => {
        console.log("state.keymap: ", state.keymap);
        if (haveRendered.current) {
            haveRendered.current = false;
            document.getElementById("code-editor-container")!.replaceChildren();
        }
        let extensions: Extension[] = [];
        if (state.keymap === "vim") {
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
                if (v.docChanged) {
                    const payload = v.state.doc.toString();
                    if (timer.current) {
                        clearTimeout(timer.current);
                    }
                    timer.current = setTimeout(() => {
                        sendToSwift(updateHandler, payload);
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

        const _view = new EditorView({
            state: startState,
            parent: document.getElementById("code-editor-container")!,
        });
        _view.focus();
        view.current = _view;
        haveRendered.current = true;
        /* eslint-disable-next-line  -- Don't want to run it on the other value change. */
    }, [state.baseKeymap, state.theme, state.haveSetInitialValue, state.keymap]);

    useEffect(() => {
        return () => window.localStorage.removeItem("editor-initial-value");
    }, []);
    useEventListener("set-editor-content", (e) => {
        if (view.current) {
            view.current.dispatch({
                changes: {
                    from: 0,
                    to: view.current.state.doc.length,
                    insert: e.detail,
                },
            });
        }
    });
    return <div className="h-full w-full" id="code-editor-container" />;
};

export const CodeEditor = (
    props: Omit<CodeEditorProps, "initialValue">,
): ReactNode => {
    const [initialValue, setInitialValue] = useLocalStorage(
        "editor-initial-value",
        undefined,
        {
            deserializer(value) {
                return value;
            },
            serializer(value) {
                return value;
            },
            initializeWithValue: false,
        },
    );
    useEffect(() => {
        if (!initialValue) {
            sendToSwift(SwiftHandler.requestInitialData, "");
        }
    }, [initialValue]);
    useEventListener("set-editor-content", (e) => {
        setInitialValue(e.detail);
    });
    return initialValue ? (
        <CodeEditorInner {...props} initialValue={initialValue} />
    ) : (
        <div className="w-full h-full flex flex-col justify-center items-center">
            <LoadingComponent />
        </div>
    );
};

CodeEditor.displayName = "CodeEditor";
