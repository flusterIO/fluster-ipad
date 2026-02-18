import React, { useEffect, useId, useRef, type ReactNode } from "react";
import { EditorState, Extension } from "@codemirror/state";
import { EditorView, keymap, ViewUpdate } from "@codemirror/view";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { history } from '@codemirror/commands';
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
import { SplitviewEditorWebviewActions, SplitviewEditorWebviewEvents, SplitviewEditorWebviewLocalStorageKeys } from "@/code_gen/typeshare/fluster_core_utilities";
import { AnyWebviewAction, AnyWebviewEvent, AnyWebviewStorageKey } from "@/utils/types/any_window_event";
import { CodeEditorLanguage } from "../types/code_editor_types";
import { languages } from '@codemirror/language-data';
import { bracketMatching, foldGutter, indentOnInput, syntaxTree } from '@codemirror/language';
import { autocompletion, closeBrackets, CompletionSource } from '@codemirror/autocomplete';
import { highlightActiveLine, dropCursor, rectangularSelection } from '@codemirror/view';
import { getFlusterSnippets } from "../data/snippets/fluster_snippets";
import { Table, TaskList } from "@lezer/markdown";
import { Prec } from "@codemirror/state";
import { SnippetStrategy } from "../data/snippets/snippet_types";
import { getMathSnippets } from "../data/snippets/math_snippets";
import { Tex, YAMLFrontMatter, bibtex } from "@fluster/lezer";
import { getBibtexSnippets } from "../data/snippets/bibtex_snippets";
import { scrollPlugin, sendEditorScrollDOMEvent } from "#/split_view_editor/state/hooks/use_editor_scroll_position";


interface CodeEditorProps {
    language?: CodeEditorLanguage;
    initialValue: string;
    requestNewDataAction?: AnyWebviewAction
    updateHandler?: AnyWebviewAction
    showWebviewHandler?: AnyWebviewAction
    swiftContentEvent?: AnyWebviewEvent
    containerId?: string;
    initialValueStorageKey?: AnyWebviewStorageKey
}



export const CodeEditorInner = ({
    language = CodeEditorLanguage.markdown,
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
    const viewRef = useRef<EditorView | null>(null)




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
        if (language === CodeEditorLanguage.markdown) {
            const mdxCompletionSource: CompletionSource = (context) => {
                const word = context.matchBefore(/\w*/);
                if (!word || (word.from === word.to && !context.explicit)) return null;
                const tree = syntaxTree(context.state);
                const node = tree.resolveInner(context.pos, -1);

                // Walk up the tree to find the context
                let curr: (typeof node | null) = node;
                while (curr) {
                    console.log("curr.name: ", curr.name)
                    if (curr.name === "TexBlock" || curr.name === "TexInline") {
                        return {
                            from: word.from,
                            options: getMathSnippets().map((f) => f.completion),
                            filter: true
                        };
                    }
                    /* if (curr.name.includes("JSX") || curr.name.includes("Tag")) { */
                    /*   return { */
                    /*     from: context.pos, */
                    /*     options: getFlusterSnippets */
                    /*   }; */
                    /* } */
                    curr = curr.parent;
                }
                return {
                    from: word.from,
                    options: getFlusterSnippets({
                        base: undefined,
                        citationKeys: state.allCitationIds,
                        includeEmojiSnippets: state.snippetProps.includeEmojiSnippets
                    }).filter((x) => x.strategy === SnippetStrategy.noLeadingText).map((n) => n.completion),
                    filter: true,

                };
            }
            extensions = [
                ...extensions,
                markdown({
                    base: markdownLanguage,
                    codeLanguages: languages,
                    extensions: [
                        Table,
                        Tex,
                        TaskList,
                        YAMLFrontMatter
                    ]
                }),
                bracketMatching(),
                /* javascript({ jsx: true }), */
                closeBrackets(),
                autocompletion(),
                Prec.high(markdownLanguage.data.of({
                    autocomplete: mdxCompletionSource
                })),
                foldGutter(),
                indentOnInput(),
                highlightActiveLine(),
                dropCursor(),
                rectangularSelection(),
            ]
        } else if (language === CodeEditorLanguage.bibtex) {
            const bt = bibtex({
                additionalSnippets: getBibtexSnippets()
            });
            extensions = [
                ...extensions,
                bt,
            ]
        }
        if (state.lockEditorScrollToPreview) {
            extensions.push(scrollPlugin)
        }
        extensions = [
            ...extensions,
            keymap.of(codeEditorBaseKeymapMap[state.baseKeymap]()),
            EditorState.allowMultipleSelections.of(true),
            EditorView.lineWrapping,
            /* language, */
            history(),
            codeEditorThemeMap[state.theme](),
            // On Change Listener
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
        viewRef.current = _view;
        haveRendered.current = true;
        sendToSwift(showWebviewHandler);
        /* eslint-disable-next-line  -- Don't want to run it on the other value change. */
    }, [state.baseKeymap, state.theme, state.haveSetInitialValue, state.keymap, state.allCitationIds, state.lockEditorScrollToPreview]);

    useEventListener(swiftContentEvent as keyof WindowEventMap, (e) => {
        if (viewRef.current) {
            viewRef.current.dispatch({
                changes: {
                    from: 0,
                    to: viewRef.current.state.doc.length,
                    insert: "detail" in e ? e.detail : "",
                },
            });
        }
    });


    useEventListener("request-editor-scroll-proportion", () => {
        if (viewRef.current?.scrollDOM) {
            sendEditorScrollDOMEvent(viewRef.current.scrollDOM!)
        }
    })

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
        /* eslint-disable-next-line  -- I hate this rule */
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
