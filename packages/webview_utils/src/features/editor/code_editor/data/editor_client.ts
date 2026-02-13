import { MdxPreviewWebviewActions } from "@/code_gen/typeshare/fluster_core_utilities";
import { sendToSwift } from "@/utils/bridge/send_to_swift";

export class EditorClient {
    static navigateToNoteById(noteId: string) {
        sendToSwift(MdxPreviewWebviewActions.ViewNoteById, noteId)
    }
    static handleTagClick(tagValue: string) {
        sendToSwift(MdxPreviewWebviewActions.OnTagClick, tagValue);
    }
}
