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
import { sendToSwift } from "@/utils/bridge/send_to_swift";
import { LoadingComponent } from "@/shared_components/loading_component";
import { useLocalStorage } from "@/state/hooks/use_local_storage";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { setWindowBridgeFunctions } from "../types/swift_events/swift_events";
import { SplitviewEditorWebviewActions, SplitviewEditorWebviewEvents, SplitviewEditorWebviewLocalStorageKeys } from "@/code_gen/typeshare/fluster_core_utilities";
import { AnyWebviewAction, AnyWebviewEvent, AnyWebviewStorageKey } from "@/utils/types/any_window_event";

interface CodeEditorProps {
    language?: Extension;
    initialValue: string;
    requestNewDataAction?: AnyWebviewAction
    updateHandler?: AnyWebviewAction
    showWebviewHandler?: AnyWebviewAction
    swiftContentEvent?: AnyWebviewEvent
    containerId?: string;
    initialValueStorageKey?: AnyWebviewStorageKey
}

setWindowBridgeFunctions();

export const CodeEditorInner = ({
    language = markdown(),
    initialValue,
    updateHandler = SplitviewEditorWebviewActions.OnEditorChange,
    showWebviewHandler = SplitviewEditorWebviewActions.SetWebviewLoaded,
    swiftContentEvent = SplitviewEditorWebviewEvents.SetSplitviewEditorContent,
    containerId = "code-editor-container"
}: CodeEditorProps): ReactNode => {
    const haveRendered = useRef(false);
    const state = useCodeEditorContext();
    const dispatch = useCodeEditorDispatch();
    const timer = useRef<NodeJS.Timeout | null>(null);
    const view = useRef<EditorView | null>(null);

    useEffect(() => {
        const em = document.getElementById(containerId)!
        if (haveRendered.current) {
            haveRendered.current = false;
            em.replaceChildren();
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
                    if (payload !== initialValue) {
                        timer.current = setTimeout(() => {
                            sendToSwift(updateHandler, payload);
                        }, 500);
                    }
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
            parent: em,
        });
        _view.focus();
        view.current = _view;
        haveRendered.current = true;
        sendToSwift(showWebviewHandler);
        /* eslint-disable-next-line  -- Don't want to run it on the other value change. */
    }, [state.baseKeymap, state.theme, state.haveSetInitialValue, state.keymap]);

    useEventListener(swiftContentEvent as keyof WindowEventMap, (e) => {
        if (view.current) {
            view.current.dispatch({
                changes: {
                    from: 0,
                    to: view.current.state.doc.length,
                    insert: "detail" in e ? e.detail : "",
                },
            });
        }
    });

    return <div className="h-full w-full" id={containerId} />;
};

export const CodeEditor = (
    props: Omit<CodeEditorProps, "initialValue">,
): ReactNode => {
    const [initialValue, setInitialValue] = useLocalStorage(
        props.initialValueStorageKey ?? SplitviewEditorWebviewLocalStorageKeys.InitialValue,
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
            sendToSwift(props.requestNewDataAction ?? SplitviewEditorWebviewActions.RequestSplitviewEditorData);
        }
    }, [initialValue]);
    useEventListener((props.swiftContentEvent ?? SplitviewEditorWebviewEvents.SetSplitviewEditorContent) as keyof WindowEventMap, (e) => {
        setInitialValue("detail" in e ? e.detail : "");
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
