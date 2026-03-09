import { type MdxEditorAppState } from "#/webview_global_state/mdx_editor/store";
import { type EditorState, SplitviewEditorWebviewActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { sendToSwift } from "@/utils/bridge/send_to_swift";
import { type AnyWebviewAction } from "@/utils/types/any_window_event";
import { useState, useEffectEvent, useEffect, type ReactNode } from "react";
import { connect } from 'react-redux';

const connector = connect((state: MdxEditorAppState) => ({
    noteId: state.editor.note_id,
}))

export const useMdxEditorValueObserver = connector(({ noteId, requestNewDataAction }: { noteId: EditorState["note_id"], requestNewDataAction?: AnyWebviewAction }): ReactNode => {
    const [initialRender, setInitialRender] = useState(true)

    const handleInitialRender = useEffectEvent(() => { setInitialRender(false); })

    useEffect(() => {
        if (!noteId || initialRender) {
            sendToSwift(requestNewDataAction ?? SplitviewEditorWebviewActions.RequestSplitviewEditorData);
        }
        handleInitialRender()

    }, [noteId, initialRender]);

    return null
})
