import React, {
    ReactNode,
    createContext,
    useReducer,
    useContext,
    useEffect,
} from "react";
import {
    CodeEditorBaseKeymap,
    CodeEditorTheme,
    stringToCodeEditorTheme,
} from "../types/code_editor_types";
import { useLocalStorage } from "@/state/hooks/use_local_storage";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { sendToSwift } from "@/utils/bridge/send_to_swift";
import { EditorStateActions, EditorSaveMethod, SplitviewEditorWebviewActions, SplitviewEditorWebviewEvents, SplitviewEditorWebviewLocalStorageKeys, SetEditorInitialStateEditorAction, SetEditorSaveMethodEditorAction, EditorView } from "@/code_gen/typeshare/fluster_core_utilities";
import { CitationResultBuffer, ParsedMdxDataTypescriptSafe, TagResultBuffer } from "@/code_gen/flat_buffer/mdx-serialization";
import { useMediaQuery } from "react-responsive";
import { GetSnippetProps } from "../data/snippets/snippet_types";
import { AnyCrossLanguageEditorAction } from "#/split_view_editor/state/cross_language_state/cross_language_state_types";


declare global {
    interface WindowEventMap {
        [SplitviewEditorWebviewEvents.EditorStateUpdate]: CustomEvent<string>;
    }
}

type SnippetPropsState = Required<Omit<GetSnippetProps, "base" | "citationKeys">>

export interface CodeEditorState {
    /**
     * Required for verification before saving manually as the async, back-forth approach with the AI parser
     * might allow tme for things to change.
     * This might resolve some DB issues that popped up during dev too... not sure if they're just dev tool things or real issues.
     */
    note_id: string | null;
    keymap: string;
    baseKeymap: CodeEditorBaseKeymap;
    theme: CodeEditorTheme;
    citations: CitationResultBuffer[]
    tags: TagResultBuffer[]
    allCitationIds: string[];
    value: string;
    /** pre-parsed editor content. */
    parsedValue: string | null;
    haveSetInitialValue: boolean;
    editorView: EditorView
    snippetProps: SnippetPropsState
    lockEditorScrollToPreview: boolean
    /// Everything below here has been moved to the new state management setup with Swift passing partial state directly instead of random data that needs to be parsed individually.
    saveMethod: EditorSaveMethod
}

const defaultInitialCodeEditorState: CodeEditorState = {
    note_id: null,
    baseKeymap: CodeEditorBaseKeymap.Default,
    theme: CodeEditorTheme.Dracula,
    keymap: "base",
    value: "",
    citations: [],
    allCitationIds: [],
    tags: [],
    parsedValue: null,
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
    saveMethod: EditorSaveMethod.OnChange
};

export const CodeEditorContext = createContext<CodeEditorState>(
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
        payload: string;
    }
    | {
        type: "setParsedEditorContentString",
        payload: string
    } | {
        type: "setEditorValue";
        payload: string;
    }
    | {
        type: "setParsedEditorContent";
        payload: ParsedMdxDataTypescriptSafe;
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
        type: "setLockEditorScrollToPreview",
        payload: boolean
    } | AnyCrossLanguageEditorAction;

export const CodeEditorDispatchContext = createContext<
    React.Dispatch<CodeEditorContextActions>
>(null!);

export const useCodeEditorContext = () => useContext(CodeEditorContext);
export const useCodeEditorDispatch = () =>
    useContext(CodeEditorDispatchContext);

export const CodeEditorContextReducer = (
    state: CodeEditorState,
    action: CodeEditorContextActions,
): CodeEditorState => {
    switch (action.type) {
        case EditorStateActions.SetEditorSaveMethod: {
            return {
                ...state,
                saveMethod: (action as SetEditorSaveMethodEditorAction).payload
            }
        }
        case EditorStateActions.SetInitialEditorState: {
            return {
                ...state,
                ...(action as SetEditorInitialStateEditorAction).payload
            }
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
        case "setParsedEditorContent": {
            const content = action.payload.parsedContent()
            const citations: CitationResultBuffer[] = [];
            for (let i = 0; i < action.payload.citationsLength(); i++) {
                const cit = action.payload.citations(i)
                if (cit) {
                    citations.push()
                }
            }
            const tags: TagResultBuffer[] = []
            for (let i = 0; i < action.payload.tagsLength(); i++) {
                const tag = action.payload.tags(i)
                if (tag) {
                    tags.push()
                }
            }
            return {
                ...state,
                parsedValue: content ? content : state.parsedValue,
                citations,
                tags
            };
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
    initialValues?: Partial<CodeEditorState>;
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
    initialValueKey = SplitviewEditorWebviewLocalStorageKeys.InitialValue,
    implementation
}: CodeEditorProviderProps) => {
    const [state, dispatch] = useReducer(
        CodeEditorContextReducer,
        initialValues
            ? { ...initialValues, ...defaultInitialCodeEditorState }
            : defaultInitialCodeEditorState,
    );

    useEventListener(SplitviewEditorWebviewEvents.EditorStateUpdate, (e) => {
        dispatch(JSON.parse(e.detail) as AnyCrossLanguageEditorAction)
    })

    const isEditorView = useMediaQuery({
        orientation: "landscape",
    });

    useEffect(() => {
        dispatch({
            type: "setEditorView",
            payload: isEditorView ? EditorView.Splitview : EditorView.PreviewOnly
        })
    }, [isEditorView])

    const [editorKeymap] = useLocalStorage(SplitviewEditorWebviewLocalStorageKeys.EditorKeymap, "base", {
        deserializer(value) {
            return value;
        },
        serializer(value) {
            return value;
        },
        initializeWithValue: false,
    });
    useEffect(() => {
        dispatch({
            type: "setKeymap",
            payload: editorKeymap,
        });
    }, [editorKeymap]);

    useEventListener(SplitviewEditorWebviewEvents.SetEditorKeymap, (e) => {
        dispatch({
            type: "setKeymap",
            payload: e.detail,
        });
    });

    const [editorTheme] = useLocalStorage(SplitviewEditorWebviewLocalStorageKeys.CodeTheme, undefined, {
        deserializer(value) {
            return value;
        },
        serializer(value) {
            return value;
        },
        initializeWithValue: false,
    });
    const handleTheme = (t: string): void => {
        const payload = stringToCodeEditorTheme((t as string) ?? "dracula");
        dispatch({
            type: "setTheme",
            payload,
        });
    };
    useEffect(() => {
        handleTheme(editorTheme);
    }, [editorTheme]);
    /* useEventListener(SplitviewEditorWebviewEvents.SetCodeTheme, (e) => { */
    /*     handleTheme(e.detail); */
    /* }); */

    const [initialValue] = useLocalStorage(initialValueKey, undefined, {
        deserializer(value) {
            return value;
        },
        serializer(value) {
            return value;
        },
        initializeWithValue: false,
    });
    useEffect(() => {
        if (!state.haveSetInitialValue && typeof initialValue === "string") {
            dispatch({
                type: "setEditorValue",
                payload: initialValue,
            });
        }
    }, [initialValue, state.haveSetInitialValue]);

    /* useEventListener(SplitviewEditorWebviewEvents.SetParsedMdxContent, (e) => { */
    /*     dispatch({ */
    /*         type: "setParsedEditorContent", */
    /*         payload: e.detail, */
    /*     }); */
    /* }); */

    useEventListener(SplitviewEditorWebviewEvents.SetParsedMdxContentString, (e) => {
        dispatch({
            type: "setParsedEditorContentString",
            payload: e.detail
        })
    })

    useEventListener(SplitviewEditorWebviewEvents.SetEditorSnippetProps, (e) => {
        const ids = []
        for (let i = 0; i <= e.detail.citationIdsLength(); i++) {
            ids.push(e.detail.citationIds(i))
        }
        dispatch({
            type: "setAllCitationIds",
            payload: ids
        })
    })

    useEventListener(SplitviewEditorWebviewEvents.SetWebviewPreviewScrollLock, (e) => {
        console.log(`Received event preview-editor scroll lock event: `, e.detail)
        dispatch({
            type: "setLockEditorScrollToPreview",
            payload: e.detail
        })
    })

    useEffect(() => {
        if (typeof state.parsedValue !== "string" && implementation === "mdx-editor") {
            sendToSwift(SplitviewEditorWebviewActions.RequestSplitviewEditorData);
        }
    }, [state.parsedValue]);




    return (
        <CodeEditorContext.Provider value={state}>
            <CodeEditorDispatchContext.Provider value={dispatch}>
                {children}
            </CodeEditorDispatchContext.Provider>
        </CodeEditorContext.Provider>
    );
};


export {
    EditorView
}
