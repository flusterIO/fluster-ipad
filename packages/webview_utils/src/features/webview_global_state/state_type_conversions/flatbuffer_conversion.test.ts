import { describe, expect, it } from "vitest";
import { editorViewRustToFlatBuffer, editorViewFlatBufferToRustType, saveMethodFlatBuffersToRustType } from "./flatbuffer_conversions";
import { EditorSaveMethodReflection, EditorViewReflection } from "@/code_gen/flat_buffer/state-reflection";
import { EditorSaveMethod, EditorView } from "@/code_gen/typeshare/fluster_core_utilities";

describe("Flatbuffer enum conversions working as expected", () => {
    it("Converts flatbuffer editorView to Rust editor view", () => {
        expect(editorViewFlatBufferToRustType(EditorViewReflection.PreviewOnly), "Matches previewOnly").toEqual(EditorView.PreviewOnly)
        expect(editorViewFlatBufferToRustType(EditorViewReflection.Splitview), "Matches previewOnly").toEqual(EditorView.Splitview)
        expect(editorViewFlatBufferToRustType(EditorViewReflection.Pending), "Matches previewOnly").toEqual(EditorView.Pending)
    })

    it("Converts editorView from Rust editor view to flatbuffers", () => {
        expect(editorViewRustToFlatBuffer(EditorView.PreviewOnly), "Matches previewOnly").toEqual(EditorViewReflection.PreviewOnly)
        expect(editorViewRustToFlatBuffer(EditorView.Splitview), "Matches previewOnly").toEqual(EditorViewReflection.Splitview)
        expect(editorViewRustToFlatBuffer(EditorView.Pending), "Matches previewOnly").toEqual(EditorViewReflection.Pending)
    })

    it("Converts saveMethod from flatBuffers to Rust type", () => {
        expect(saveMethodFlatBuffersToRustType(EditorSaveMethodReflection.OnChange), "Equals 'onChange'").toEqual(EditorSaveMethod.OnChange)
        expect(saveMethodFlatBuffersToRustType(EditorSaveMethodReflection.OnSave), "Equals 'onChange'").toEqual(EditorSaveMethod.OnSave)
    })
})
