import { setEditorView } from "#/webview_global_state/mdx_editor/state/editor_state_slice";
import { EditorView } from "@/code_gen/typeshare/fluster_core_utilities";
import { useEffect } from "react";
import { useDispatch } from "react-redux";
import { useMediaQuery } from "react-responsive";

export const useCodeEditorView = () => {
    const dispatch = useDispatch()
    const isEditorView = useMediaQuery({
        orientation: "landscape",
    });

    useEffect(() => {
        dispatch(setEditorView(isEditorView ? EditorView.Splitview : EditorView.PreviewOnly))
    }, [isEditorView])
}
