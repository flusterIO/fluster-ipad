import React, {
    ReactNode,
    createContext,
    useReducer,
    useContext,
    useEffect,
} from "react";
import json from 'superjson';
import {
    CodeEditorBaseKeymap,
    CodeEditorTheme,
    stringToCodeEditorTheme,
} from "../types/code_editor_types";
import { useLocalStorage } from "@/state/hooks/use_local_storage";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { sendToSwift } from "@/utils/bridge/send_to_swift";
import { EditorStateActions, EditorSaveMethod, SplitviewEditorWebviewActions, SplitviewEditorWebviewEvents, SplitviewEditorWebviewLocalStorageKeys, EditorView, EditorState, CodeEditorKeymap, EditorCitation } from "@/code_gen/typeshare/fluster_core_utilities";
import { useMediaQuery } from "react-responsive";
import { GetSnippetProps } from "../data/snippets/snippet_types";
import { AnyCrossLanguageBufferEditorAction, AnyCrossLanguageEditorAction } from "#/split_view_editor/state/cross_language_state/cross_language_state_types";
import { BindEditorStateToLocalStorage } from "./bind_editor_state_to_local_storage";
import { OnParsedContentChangeEventBuffer } from "@/code_gen/flat_buffer/mdx-serialization";
import { ByteBuffer } from "flatbuffers";
import { useCodeEditorView } from "./hooks/use_editor_view_type";


declare global {
    interface WindowEventMap {
        [SplitviewEditorWebviewEvents.EditorStateUpdate]: CustomEvent<string>;
        [SplitviewEditorWebviewEvents.EditorStateParsedContentUpdate]: CustomEvent<number[]>;
    }
}

type SnippetPropsState = Required<Omit<GetSnippetProps, "base" | "citationKeys">>


const defaultInitialCodeEditorState: EditorState = {
    note_id: undefined,
    baseKeymap: CodeEditorBaseKeymap.Default,
    theme: CodeEditorTheme.Dracula,
    keymap: CodeEditorKeymap.Base,
    value: "",
    citations: [],
    allCitationIds: [],
    tags: [],
    parsedValue: undefined,
    haveSetInitialValue: false,
    editorView: EditorView.Pending,
    snippetProps: {
        // Default snippet state.
        includeEmojiSnippets: true
    },
    lockEditorScrollToPreview: false,
    /**
     * Leave this set to onChange by default to encourage user's to explore the app early on, but mention in the opening note that it can be changed.
     */
    saveMethod: EditorSaveMethod.OnChange,
    autoSaveTimeout: 500
};

export const CodeEditorContext = createContext<EditorState>(
    defaultInitialCodeEditorState,
);

export type CodeEditorContextActions =
    | {
        type: "setValue";
        payload: string;
    }
    | {
        type: "setTheme";
        payload: CodeEditorTheme;
    }
    | {
        type: "setBaseKeymap";
        payload: CodeEditorBaseKeymap;
    }
    | {
        type: "setKeymap";
        payload: CodeEditorKeymap;
    }
    | {
        type: "setParsedEditorContentString",
        payload: string
    } | {
        type: "setEditorValue";
        payload: string;
    } | {
        type: "setAllCitationIds";
        payload: string[]
    } | {
        type: "setEditorView",
        payload: EditorView
    } | {
        type: "setSnippetProps",
        payload: SnippetPropsState
    } | {
        type: "loadSavedState",
        payload: EditorState
    } | {
        type: "setLockEditorScrollToPreview",
        payload: boolean
    } | AnyCrossLanguageBufferEditorAction | AnyCrossLanguageEditorAction;

export const CodeEditorDispatchContext = createContext<
    React.Dispatch<CodeEditorContextActions>
>(null!);

export const useCodeEditorContext = () => useContext(CodeEditorContext);
export const useCodeEditorDispatch = () =>
    useContext(CodeEditorDispatchContext);

export const CodeEditorContextReducer = (
    state: EditorState,
    action: CodeEditorContextActions,
): EditorState => {
    console.log("Editor Action: ", action)
    switch (action.type) {
        case EditorStateActions.SetEditorSaveMethod: {
            return {
                ...state,
                saveMethod: action.payload
            }
        }
        case EditorStateActions.SetInitialEditorState: {
            return {
                ...state,
                ...action.payload
            }
        }
        case EditorStateActions.SetParsedEditorContent: {
            const parsedContent = action.payload.parsedContent() ?? state.parsedValue
            const citations: EditorCitation[] = []
            for (let i = 0; i < action.payload.citationsLength(); i++) {
                const cit = action.payload.citations(i)
                const citation_key = cit?.citationKey()
                const html = cit?.html()
                if (cit && citation_key && html) {
                    citations.push({
                        html,
                        citation_key
                    })
                }
            }
            return {
                ...state,
                parsedValue: parsedContent,
                citations: citations
            }
        }
        case "loadSavedState": {
            return action.payload
        }
        /// TODO: Handle setting of all of these types with new EditorStateHandler and the Swift partial-state approach..
        case "setValue": {
            return {
                ...state,
                haveSetInitialValue: true,
                value: action.payload,
            };
        }
        case "setTheme": {
            return {
                ...state,
                theme: action.payload,
            };
        }
        case "setKeymap": {
            return {
                ...state,
                keymap: action.payload,
            };
        }
        case "setBaseKeymap": {
            return {
                ...state,
                baseKeymap: action.payload,
            };
        }
        case "setEditorValue": {
            return {
                ...state,
                haveSetInitialValue: true,
                value: action.payload,
            };
        }
        case "setParsedEditorContentString": {
            return {
                ...state,
                haveSetInitialValue: true,
                parsedValue: action.payload
            }
        }
        case "setAllCitationIds": {
            return {
                ...state,
                allCitationIds: action.payload
            }
        }
        case "setEditorView": {
            return {
                ...state,
                editorView: action.payload
            }
        }
        case "setSnippetProps": {
            return {
                ...state,
                snippetProps: {
                    ...state.snippetProps,
                    ...action.payload
                }
            }
        }
        case "setLockEditorScrollToPreview": {
            return {
                ...state,
                lockEditorScrollToPreview: action.payload
            }
        }
    }
};

CodeEditorContextReducer.displayName = "CodeEditorContextReducer";


export type CodeEditorImplementation = "bib-editor" | "mdx-editor" | "mdx-viewer" | "development"

interface CodeEditorProviderProps {
    children: ReactNode;
    initialValues?: Partial<EditorState>;
    initialValueKey?: string;
    implementation: CodeEditorImplementation
}



/**
 * Still need to add the following methods from Swift:
 * - [ ] setSnippetProps 
 *   - Still need to create action in rust, flat buffer type and all serialization from Swift.
 */
export const CodeEditorProvider = ({
    children,
    initialValues,
    /* initialValueKey = SplitviewEditorWebviewLocalStorageKeys.InitialValue, */
    implementation
}: CodeEditorProviderProps) => {
    const [state, dispatch] = useReducer(
        CodeEditorContextReducer,
        initialValues
            ? { ...initialValues, ...defaultInitialCodeEditorState }
            : defaultInitialCodeEditorState,
    );

    useCodeEditorView(dispatch)

    useEventListener(SplitviewEditorWebviewEvents.EditorStateUpdate, (e) => {
        dispatch(json.parse(e.detail) as AnyCrossLanguageEditorAction)
    })

    useEventListener(SplitviewEditorWebviewEvents.EditorStateParsedContentUpdate, (e) => {

        const data = Uint8Array.from(e.detail)
        const buf = new ByteBuffer(data)
        const onChangeEvent = OnParsedContentChangeEventBuffer.getRootAsOnParsedContentChangeEventBuffer(buf)
        dispatch({
            type: EditorStateActions.SetParsedEditorContent,
            payload: onChangeEvent
        })
    })

    /* const [editorKeymap] = useLocalStorage<CodeEditorKeymap>(SplitviewEditorWebviewLocalStorageKeys.EditorKeymap, CodeEditorKeymap.Base, { */
    /*     initializeWithValue: false, */
    /* }); */
    /* useEffect(() => { */
    /*     dispatch({ */
    /*         type: "setKeymap", */
    /*         payload: editorKeymap as CodeEditorKeymap, */
    /*     }); */
    /* }, [editorKeymap]); */

    /* useEventListener(SplitviewEditorWebviewEvents.SetEditorKeymap, (e) => { */
    /*     dispatch({ */
    /*         type: "setKeymap", */
    /*         payload: e.detail, */
    /*     }); */
    /* }); */
    /* const [editorTheme] = useLocalStorage(SplitviewEditorWebviewLocalStorageKeys.CodeTheme, undefined, { */
    /*     deserializer(value) { */
    /*         return value; */
    /*     }, */
    /*     serializer(value) { */
    /*         return value; */
    /*     }, */
    /*     initializeWithValue: false, */
    /* }); */
    /* const handleTheme = (t: string): void => { */
    /*     const payload = stringToCodeEditorTheme((t as string) ?? "dracula"); */
    /*     dispatch({ */
    /*         type: "setTheme", */
    /*         payload, */
    /*     }); */
    /* }; */
    /* useEffect(() => { */
    /*     handleTheme(editorTheme); */
    /* }, [editorTheme]); */
    /* useEventListener(SplitviewEditorWebviewEvents.SetCodeTheme, (e) => { */
    /*     handleTheme(e.detail); */
    /* }); */

    /* const [initialValue] = useLocalStorage(initialValueKey, undefined, { */
    /*     deserializer(value) { */
    /*         return value; */
    /*     }, */
    /*     serializer(value) { */
    /*         return value; */
    /*     }, */
    /*     initializeWithValue: false, */
    /* }); */
    /* useEffect(() => { */
    /*     if (!state.haveSetInitialValue && typeof initialValue === "string") { */
    /*         dispatch({ */
    /*             type: "setEditorValue", */
    /*             payload: initialValue, */
    /*         }); */
    /*     } */
    /* }, [initialValue, state.haveSetInitialValue]); */

    /* useEventListener(SplitviewEditorWebviewEvents.SetParsedMdxContent, (e) => { */
    /*     dispatch({ */
    /*         type: "setParsedEditorContent", */
    /*         payload: e.detail, */
    /*     }); */
    /* }); */

    /* useEventListener(SplitviewEditorWebviewEvents.SetParsedMdxContentString, (e) => { */
    /*     dispatch({ */
    /*         type: "setParsedEditorContentString", */
    /*         payload: e.detail */
    /*     }) */
    /* }) */

    /* useEventListener(SplitviewEditorWebviewEvents.SetEditorSnippetProps, (e) => { */
    /*     const ids = [] */
    /*     for (let i = 0; i <= e.detail.citationIdsLength(); i++) { */
    /*         ids.push(e.detail.citationIds(i)) */
    /*     } */
    /*     dispatch({ */
    /*         type: "setAllCitationIds", */
    /*         payload: ids */
    /*     }) */
    /* }) */

    /* useEventListener(SplitviewEditorWebviewEvents.SetWebviewPreviewScrollLock, (e) => { */
    /*     console.log(`Received event preview-editor scroll lock event: `, e.detail) */
    /*     dispatch({ */
    /*         type: "setLockEditorScrollToPreview", */
    /*         payload: e.detail */
    /*     }) */
    /* }) */

    useEffect(() => {
        if (typeof state.parsedValue !== "string" && implementation === "mdx-editor") {
            sendToSwift(SplitviewEditorWebviewActions.RequestSplitviewEditorData);
        }
    }, [state.parsedValue]);




    return (
        <CodeEditorContext.Provider value={state}>
            <CodeEditorDispatchContext.Provider value={dispatch}>
                <BindEditorStateToLocalStorage />
                {children}
            </CodeEditorDispatchContext.Provider>
        </CodeEditorContext.Provider>
    );
};


export {
    EditorView
}
