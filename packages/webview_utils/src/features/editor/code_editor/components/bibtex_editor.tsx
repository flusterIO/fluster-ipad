import React, { type ReactNode } from "react";
import { CodeEditorInner } from "./code_editor";
import { LoadingComponent } from "@/shared_components/loading_component";
import { setBibtexEditorWindowBridgeFunctions } from "../types/swift_events/bibtex_editor_swift_events";
import { BibtexEditorWebviewActions, BibtexEditorWebviewEvents, BibtexEditorWebviewLocalStorageKeys, type EditorState } from "@/code_gen/typeshare/fluster_core_utilities";
import { CodeEditorLanguage } from "../types/code_editor_types";
import { connect } from 'react-redux';
import { type MdxEditorAppState } from "#/webview_global_state/mdx_editor/store";

setBibtexEditorWindowBridgeFunctions();

const connector = connect((state: MdxEditorAppState) => ({
    havSetInitialValue: state.editor.haveSetInitialValue,
}))


export const BibtexEditor = connector(({ haveSetInitialValue }: Pick<EditorState, "haveSetInitialValue">): ReactNode => {
    /* const dispatch = useDispatch() */
    /* useEventListener(BibtexEditorWebviewEvents.SetBibtexEditorContent, (e) => { */
    /*     dispatch(handleEditorChange(e.detail)) */
    /* }); */

    /* useEffect(() => { */
    /*     if (!haveSetInitialValue) { */
    /*         sendToSwift(BibtexEditorWebviewActions.RequestBibtexEditorData, ""); */
    /*     } */
    /* }, [haveSetInitialValue]); */

    return haveSetInitialValue ? (
        <CodeEditorInner
            language={CodeEditorLanguage.bibtex}
            requestNewDataAction={BibtexEditorWebviewActions.RequestBibtexEditorData}
            updateHandler={BibtexEditorWebviewActions.OnEditorChange}
            showWebviewHandler={BibtexEditorWebviewActions.SetWebviewLoaded}
            initialValueStorageKey={BibtexEditorWebviewLocalStorageKeys.InitialValue}
            swiftContentEvent={BibtexEditorWebviewEvents.SetBibtexEditorContent}
        />
    ) : (
        <div className="w-full h-full flex flex-col justify-center items-center">
            <LoadingComponent />
        </div>
    );
});

BibtexEditor.displayName = "BibtexEditor";
