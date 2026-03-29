import React, { type ComponentProps, useEffect, useRef, type ReactNode } from "react";
import { type Extension, EditorState as CodeMirrorEditorState } from "@codemirror/state";
import { EditorView, keymap as codemirrorKeymap, type ViewUpdate } from "@codemirror/view";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { history } from '@codemirror/commands';
import { vim } from "@replit/codemirror-vim";
import { codeEditorBaseKeymapMap } from "../data/code_editor_base_keymap_map";
import { codeEditorThemeMap } from "../data/code_editor_theme_map";
import { lineNumbersRelative } from "@uiw/codemirror-extensions-line-numbers-relative";
import { sendToSwift } from "@/utils/bridge/send_to_swift";
import { LoadingComponent } from "@/shared_components/loading_component";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { type BibtexEditorWebviewEvents, CodeEditorKeymap, type EditorState, SplitviewEditorWebviewActions, SplitviewEditorWebviewEvents, type WebviewContainerState } from "@/code_gen/typeshare/fluster_core_utilities";
import { type AnyWebviewAction, type AnyWebviewStorageKey } from "@/utils/types/any_window_event";
import { CodeEditorLanguage } from "../types/code_editor_types";
import { languages } from '@codemirror/language-data';
import { bracketMatching, foldGutter, indentOnInput, syntaxTree } from '@codemirror/language';
import { autocompletion, closeBrackets, completeFromList, type CompletionSource } from '@codemirror/autocomplete';
import { highlightActiveLine, dropCursor, rectangularSelection } from '@codemirror/view';
import { getFlusterSnippets } from "../data/snippets/fluster_snippets";
import { Prec } from "@codemirror/state";
import { type GetSnippetProps, SnippetStrategy } from "../data/snippets/snippet_types";
import { getMathSnippets } from "../data/snippets/math_snippets";
import { Tex } from "@fluster/lezer";
import { scrollPlugin, sendEditorScrollDOMEvent } from "#/split_view_editor/state/hooks/use_editor_scroll_position";
import { getBibtexSnippets } from "../data/snippets/bibtex_snippets";
import { bibtexLanguage, bibtex } from "@citedrive/codemirror-lang-bibtex"
import { EditorClient } from "../data/editor_client";
import { useDispatch } from 'react-redux';
import { connect } from "react-redux"
import { setBibtexEditorValue, setEditorValue } from "#/webview_global_state/mdx_editor/state/editor_state_slice";
import { type GlobalAppState } from "#/webview_global_state/store";
import { type WithNullableOptionals } from "../../../../core/utils/types/utility_types";
import { cn } from "../../../../core/utils/cn";

const connector = connect((state: GlobalAppState) => ({
    baseKeymap: state.editor.baseKeymap,
    allCitationIds: state.editor.allCitationIds,
    theme_dark: state.editor.theme_dark,
    theme_light: state.editor.theme_light,
    dark_mode: state.container.dark_mode,
    keymap: state.editor.keymap,
    value: state.editor.value,
    lockEditorScrollToPreview: state.editor.lockEditorScrollToPreview,
    snippetProps: state.editor.snippetProps,
    note_id: state.editor.note_id,
    haveSetInitialValue: state.editor.haveSetInitialValue,
    autoSaveTimeout: state.editor.autoSaveTimeout,
    bibtexValue: state.editor.bib_editor.value,
    fontSize: state.container.font_size
}))


interface CodeEditorProps extends Pick<WithNullableOptionals<EditorState>, "baseKeymap" | "allCitationIds" | "theme_light" | "theme_dark" | "keymap" | "value" | "lockEditorScrollToPreview" | "snippetProps" | "note_id" | "haveSetInitialValue" | "autoSaveTimeout">, Pick<WithNullableOptionals<WebviewContainerState>, "dark_mode"> {
    language?: CodeEditorLanguage;
    requestNewDataAction?: AnyWebviewAction
    updateHandler?: AnyWebviewAction
    showWebviewHandler?: AnyWebviewAction
    swiftContentEvent?: SplitviewEditorWebviewEvents.SetSplitviewEditorContent | BibtexEditorWebviewEvents.SetBibtexEditorContent
    containerId?: string;
    initialValueStorageKey?: AnyWebviewStorageKey
    bibtexValue: string
    fontSize: WebviewContainerState["font_size"]
}


export const CodeEditorInner = connector(({
    language = CodeEditorLanguage.markdown,
    showWebviewHandler = SplitviewEditorWebviewActions.SetWebviewLoaded,
    swiftContentEvent = SplitviewEditorWebviewEvents.SetSplitviewEditorContent,
    containerId = "code-editor-container",
    keymap,
    baseKeymap,
    theme_light,
    theme_dark,
    value,
    dark_mode,
    haveSetInitialValue,
    lockEditorScrollToPreview,
    snippetProps,
    note_id,
    allCitationIds,
    bibtexValue,
    fontSize
}: CodeEditorProps): ReactNode => {
    const haveRendered = useRef(false);
    const timer = useRef<NodeJS.Timeout | null>(null);
    const viewRef = useRef<EditorView | null>(null)
    const dispatch = useDispatch()

    useEffect(() => {
        console.log("dark_mode: ", dark_mode)
        const em = document.getElementById(containerId);
        if (!em) {
            return
        }
        if (haveRendered.current) {
            haveRendered.current = false;
            em.replaceChildren();
        }
        let extensions: Extension[] = [];
        if (keymap === CodeEditorKeymap.Vim) {
            extensions.push(vim());
            extensions.push(lineNumbersRelative);
        }
        const _snippetProps: GetSnippetProps = {
            base: undefined,
            citationKeys: allCitationIds,
            includeEmojiSnippets: snippetProps.includeEmojiSnippets
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
                    if (curr.name === "TexBlock" || curr.name === "TexInline") {
                        return {
                            from: word.from,
                            options: getMathSnippets().map((f) => f.completion),
                            filter: true
                        };
                    } else {
                        console.log("curr: ", curr)
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
                    options: getFlusterSnippets(_snippetProps).filter((x) => x.strategy === SnippetStrategy.noLeadingText).map((n) => n.completion),
                    filter: true,

                };
            }
            extensions = [
                ...extensions,
                markdown({
                    base: markdownLanguage,
                    codeLanguages: languages,
                    extensions: [
                        Tex,
                        /* Table, */
                        /* TaskList, */
                        /* YAMLFrontMatter */
                    ]
                }),
                Prec.high(markdownLanguage.data.of({
                    autocomplete: mdxCompletionSource
                })),
            ]
        } else {
            const snippets = getBibtexSnippets()
            extensions = [
                ...extensions,
                bibtex(),
                Prec.high(
                    bibtexLanguage.data.of({
                        autocomplete: completeFromList(snippets)
                    })
                )
            ]
        }
        if (lockEditorScrollToPreview) {
            extensions.push(scrollPlugin)
        }
        extensions = [
            ...extensions,
            bracketMatching(),
            closeBrackets(),
            autocompletion(),
            foldGutter(),
            indentOnInput(),
            highlightActiveLine(),
            dropCursor(),
            rectangularSelection(),
            codemirrorKeymap.of(codeEditorBaseKeymapMap[baseKeymap]()),
            CodeMirrorEditorState.allowMultipleSelections.of(true),
            EditorView.lineWrapping,
            /* language, */
            history(),
            codeEditorThemeMap[dark_mode ? theme_dark : theme_light](),
            codemirrorKeymap.of([{
                key: "Mod-s",
                run: (view) => {
                    const content = view.state.doc.toString()
                    if (note_id) {
                        EditorClient.sendManualSaveRequest({
                            note_id: note_id,
                            current_note_content: content
                        })
                        return true;
                    } else {
                        return false
                    }
                },
                preventDefault: true
            },
            ]),
            // On Change Listener
            EditorView.updateListener.of((v: ViewUpdate) => {
                if (v.docChanged) {
                    const payload = v.state.doc.toString();
                    if (timer.current) {
                        clearTimeout(timer.current);
                    }
                    dispatch(
                        language === CodeEditorLanguage.markdown ? setEditorValue(payload) : setBibtexEditorValue(payload)
                    );
                }
            }),
        ] satisfies Extension[];
        const startState = CodeMirrorEditorState.create({
            doc: language === CodeEditorLanguage.markdown ? value : bibtexValue,
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

    }, [baseKeymap, theme_light, theme_dark, haveSetInitialValue, keymap, allCitationIds, lockEditorScrollToPreview, note_id, dark_mode]);

    useEventListener(swiftContentEvent as keyof WindowEventMap, (e) => {
        if (viewRef.current) {
            viewRef.current.dispatch({
                changes: {
                    from: 0,
                    to: viewRef.current.state.doc.length,
                    insert: "detail" in e ? (e.detail as string) : "",
                },

            });
        }
    });

    useEventListener("request-editor-scroll-proportion", () => {
        if (viewRef.current?.scrollDOM) {
            sendEditorScrollDOMEvent(viewRef.current.scrollDOM)
        }
    })


    // Keep the editor text one size smaller than the content text while the size range allows.
    const oneSizeDown: Record<typeof fontSize, string> = {
        small: "prose-sm",
        base: "prose-sm",
        large: "prose-base",
        xl: "prose-lg",
        xxl: "prose-xl"
    }

    return <div className={cn("h-screen w-full mdx-editor-container", oneSizeDown[fontSize])} id={containerId} />;
});


const connectorOuter = connect((state: GlobalAppState) => ({
    note_id: state.editor.note_id,
    value: state.editor.value
}))


export const CodeEditor = connectorOuter((
    props: ComponentProps<typeof CodeEditorInner> & Pick<WithNullableOptionals<EditorState>, "value" | "note_id">,
): ReactNode => {

    return typeof props.value === "string" ? (
        <CodeEditorInner {...props} />
    ) : (
        <div className="w-full h-full flex flex-col justify-center items-center">
            <LoadingComponent />
        </div>
    );
});

CodeEditor.displayName = "CodeEditor";
