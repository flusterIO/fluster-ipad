import { EditorSaveMethodReflection, EditorViewReflection } from "@/code_gen/flat_buffer/state-reflection";
import { EditorSaveMethod, EditorView } from "@/code_gen/typeshare/fluster_core_utilities";

export const editorViewFlatBufferToRustType = (editorView: EditorViewReflection): EditorView => {
    switch (editorView) {
        case EditorViewReflection.Pending:
            return EditorView.Pending
        case EditorViewReflection.Splitview:
            return EditorView.Splitview
        case EditorViewReflection.PreviewOnly:
            return EditorView.PreviewOnly
    }
}

export const editorViewRustToFlatBuffer = (editorView: EditorView): EditorViewReflection => {
    switch (editorView) {
        case EditorView.Pending:
            return EditorViewReflection.Pending
        case EditorView.PreviewOnly:
            return EditorViewReflection.PreviewOnly
        case EditorView.Splitview:
            return EditorViewReflection.Splitview
    }
}

// Begin `savemethod` section

export const saveMethodRustToFlatBuffers = (sm: EditorSaveMethod): EditorSaveMethodReflection => {
    switch (sm) {
        case EditorSaveMethod.OnChange:
            return EditorSaveMethodReflection.OnChange
        case EditorSaveMethod.OnSave:
            return EditorSaveMethodReflection.OnSave
    }
}

export const saveMethodFlatBuffersToRustType = (sm: EditorSaveMethodReflection): EditorSaveMethod => {
    switch (sm) {
        case EditorSaveMethodReflection.OnSave:
            return EditorSaveMethod.OnSave
        case EditorSaveMethodReflection.OnChange:
            return EditorSaveMethod.OnChange
    }
}
