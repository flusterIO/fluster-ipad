import path from "path";
import { bench, describe } from "vitest";
import { editorStateToFlatBuffers } from "../editor_state_to_flat_buffers";
import { EditorState } from "@/code_gen/typeshare/fluster_core_utilities";
import fs from 'fs'



const testContentData = JSON.parse(fs.readFileSync(path.resolve(__dirname, "../tests/data/generated_editor_state_data.json"), { encoding: "utf-8" })) as EditorState[]

testContentData.forEach((testData, i) => {
    describe(`Serializing editor state ${i}`, {}, () => {
        bench(`Editor state to flatbuffers ${i}`, () => {
            editorStateToFlatBuffers(testData)
        })

        bench(`Editor state to json ${i}`, () => {
            JSON.stringify(testData)
        })
    })
})
