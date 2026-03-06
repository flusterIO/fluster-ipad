import { describe, expect, it } from "vitest";
import fs from 'fs'
import path from 'path'
import { EditorState } from "@/code_gen/typeshare/fluster_core_utilities";
import { editorStateToFlatBuffers } from "../editor_state_to_flat_buffers";
import { EditorStateReflection } from "@/code_gen/flat_buffer/state-reflection";
import { ByteBuffer } from "flatbuffers";
import { editorViewFlatBufferToRustType, saveMethodFlatBuffersToRustType } from "#/webview_global_state/state_type_conversions/flatbuffer_conversions";

const testContentData = JSON.parse(fs.readFileSync(path.resolve(__dirname, "../tests/data/generated_editor_state_data.json"), { encoding: "utf-8" })) as (EditorState & { magnitude: number })[]

(async () => {
    const arr = Array(testContentData.length).fill(0).map((_, i) => i)
    for (const itemIndex of arr) {
        const itemByIndex = testContentData[itemIndex]
        console.log("itemByIndex: ", itemByIndex)
        // itemByIndex.value = await getBenchmarkContent(10 * itemIndex)
    }
})()

describe("Serializes editor state to flatbuffers", {
    concurrent: true
}, () => {
    it("Serializes without throwing an error", () => {
        for (const testContent of testContentData) {
            const res = editorStateToFlatBuffers(testContent)
            expect(res, "Flatbuffers serialization at least returns something....")
        }
    })
    it("Returns data matching the input", () => {
        for (const testContent of testContentData) {
            const res = editorStateToFlatBuffers(testContent)
            const arr = Uint8Array.from(res)
            const buf = new ByteBuffer(arr)
            const data = EditorStateReflection.getRootAsEditorStateReflection(buf)
            expect(data.noteId(), "Flatbuffers deserialzes note_id properly").toEqual(testContent.note_id)
            expect(data.baseKeymap(), "Flatbuffers deserialzes baseKeymap properly").toEqual(testContent.baseKeymap)
            const citLength = data.citationsLength()
            expect(citLength === testContent.citations.length, "Flatbuffers finds proper citation length")
            for (let i = 0; i < citLength; i++) {
                const cit = data.citations(i)
                expect(cit, "Finds citation by index")
                expect(cit!.citationKey()).toEqual(testContent.citations[i].citation_key)
                expect(cit!.html()).toEqual(testContent.citations[i].html)
            }
            expect(data.keymap(), "Flatbuffers deserialzes keymmap properly").toEqual(testContent.keymap)
            expect(data.theme(), "Flatbuffers deserialzes theme properly").toEqual(testContent.theme)
            expect(data.tagsLength(), "Finds proper tags length").toEqual(testContent.tags.length)
            expect(data.value(), "Flatbuffers deserialzes value properly").toEqual(testContent.value)
            expect(data.parsedValue(), "Flatbuffers deserialzes parsedValue properly").toEqual(testContent.parsedValue)
            expect(data.haveSetInitialValue(), "Finds proper 'haveSetInitialValue'").toEqual(testContent.haveSetInitialValue)
            console.log("data.editorView().toString(), testContent.editorView.toString(): ", data.editorView().toString(), testContent.editorView.toString())
            expect(editorViewFlatBufferToRustType(data.editorView()), "Finds proper 'editorView'").toEqual(testContent.editorView.toString())
            const snippetProps = data.snippetProps()
            expect(snippetProps, "Parses snippet props successfully")
            expect(snippetProps?.includeEmojiSnippets(), "Matches 'includeEmojiSnippets'").toEqual(testContent.snippetProps.includeEmojiSnippets)
            expect(data.lockEditorScrollToPreview(), "Matches 'lockEditorScrollToPreview'").toEqual(testContent.lockEditorScrollToPreview)
            expect(saveMethodFlatBuffersToRustType(data.saveMethod()), "'saveMethod' matches").toEqual(testContent.saveMethod)
            expect(data.autoSaveTimeout(), "matches 'autoSaveTimeout'").toEqual(testContent.autoSaveTimeout)

        }
    })
})
