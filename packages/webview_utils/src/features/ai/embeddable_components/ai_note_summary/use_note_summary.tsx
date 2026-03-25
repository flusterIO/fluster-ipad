import { useSelector } from "react-redux"
import { type GlobalWebviewStateDeepNullable } from "../../../webview_global_state/cross_language_state_types"
import { useEffect } from "react"
import { sendToSwift } from "../../../../core/utils/bridge/send_to_swift"
import { type GenerateNoteSummaryRequest, NoteDetailEvents } from "@/code_gen/typeshare/fluster_core_utilities"

export const useNoteSummary = (forceRegenerate = false) => {
    const summary = useSelector((state: GlobalWebviewStateDeepNullable) => state.note_details?.summary)
    const note_id = useSelector((state: GlobalWebviewStateDeepNullable) => state.editor.note_id)

    useEffect(() => {
        if ((!summary || forceRegenerate) && note_id) {
            sendToSwift(NoteDetailEvents.SendGenerateSummaryRequest, JSON.stringify({
                note_id
            } satisfies GenerateNoteSummaryRequest))
        }
    }, [summary, forceRegenerate, note_id])

    return summary
}
