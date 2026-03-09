import { OnParsedContentChangeEventBuffer } from "@/code_gen/flat_buffer/mdx-serialization"
import { type EditorState } from "@/code_gen/typeshare/fluster_core_utilities"
import { ByteBuffer } from "flatbuffers"

export const handleEditorStateParsedContentUpdate = (payload: number[]): void => {
    const data = Uint8Array.from(payload)
    const buf = new ByteBuffer(data)
    const item = OnParsedContentChangeEventBuffer.getRootAsOnParsedContentChangeEventBuffer(buf)
    const citations: EditorState["citations"] = []
    const citationsLength = item.citationsLength()
    for (let i = 0; i < citationsLength; i++) {
        const cit = item.citations(i)
        if (cit) {
            const html = cit.html()
            const citationKey = cit.citationKey()
            if (html && citationKey) {
                citations.push({
                    html,
                    citation_key: citationKey
                })
            }
        }
    }

}
