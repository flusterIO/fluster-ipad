import { useEffect } from "react";
import { useMediaQuery } from "react-responsive";
import { EditorView, type useCodeEditorDispatch } from "../code_editor_provider";

export const useCodeEditorView = (dispatch: ReturnType<typeof useCodeEditorDispatch>) => {
    const isEditorView = useMediaQuery({
        orientation: "landscape",
    });

    useEffect(() => {
        dispatch({
            type: "setEditorView",
            payload: isEditorView ? EditorView.Splitview : EditorView.PreviewOnly
        })
    }, [isEditorView])
}
