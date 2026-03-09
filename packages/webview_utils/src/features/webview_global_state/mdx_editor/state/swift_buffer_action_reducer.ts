import { type AnyCrossLanguageBufferEditorAction } from "#/split_view_editor/state/cross_language_state/cross_language_state_types";
import { OnParsedContentChangeEventBuffer } from "@/code_gen/flat_buffer/mdx-serialization";
import { type EditorState } from "@/code_gen/typeshare/fluster_core_utilities";
import { type PayloadAction } from "@reduxjs/toolkit";

export const swiftBufferActionReducer = (state: EditorState, action: PayloadAction<AnyCrossLanguageBufferEditorAction>): EditorState => {
    const x = OnParsedContentChangeEventBuffer.getRootAsOnParsedContentChangeEventBuffer(action.payload.payload)
    const parsedContent = x.parsedContent() ?? ""
    const citations: EditorState["citations"] = []
    const citLength = x.citationsLength()
    for (let i = 0; i < citLength; i++) {
        const cit = x.citations(i)
        const html = cit?.html()
        const citationKey = cit?.citationKey()
        if (html && citationKey) {
            citations.push({
                html,
                citation_key: citationKey
            })
        }
    }
    state.parsedValue = parsedContent;
    state.citations = citations;
    return state
}
