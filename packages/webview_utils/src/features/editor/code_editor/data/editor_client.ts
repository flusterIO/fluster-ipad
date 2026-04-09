import { type ManualSaveRequestEvent, MdxPreviewWebviewActions, SplitviewEditorWebviewActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { sendToSwift } from "@/utils/bridge/send_to_swift";

export const EditorClient = {
    navigateToNoteById: (noteId: string) => {
        sendToSwift(MdxPreviewWebviewActions.ViewNoteById, noteId)
    },
    handleTagClick: (tagValue: string) => {
        sendToSwift(MdxPreviewWebviewActions.OnTagClick, tagValue);
    },
    sendManualSaveRequest: (data: ManualSaveRequestEvent) => {
        sendToSwift(SplitviewEditorWebviewActions.ManualSaveRequest, JSON.stringify(data))
    },
}
