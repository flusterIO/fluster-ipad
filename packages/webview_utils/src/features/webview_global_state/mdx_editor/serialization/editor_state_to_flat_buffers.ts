import { EditorCitationReflection, EditorSaveMethodReflection, EditorStateReflection, EditorTagReflection, EditorViewReflection, SnippetStateReflection } from "@/code_gen/flat_buffer/state-reflection";
import { EditorSaveMethod, EditorState, EditorView } from "@/code_gen/typeshare/fluster_core_utilities";
import { Builder } from "flatbuffers";


const editorViewReflect = (s: EditorState["editorView"]): EditorViewReflection => {
    switch (s) {
        case EditorView.Pending:
            return EditorViewReflection.Pending
        case EditorView.PreviewOnly:
            return EditorViewReflection.PreviewOnly
        case EditorView.Splitview:
            return EditorViewReflection.Splitview
    }
}

const saveMethodReflect = (s: EditorSaveMethod): EditorSaveMethodReflection => {
    switch (s) {
        case EditorSaveMethod.OnChange:
            return EditorSaveMethodReflection.OnChange
        case EditorSaveMethod.OnSave:
            return EditorSaveMethodReflection.OnSave
    }
}

/**
 * wtf... Serializing to json is actually faster for small payloads...
 */
export const editorStateToFlatBuffers = (editorState: EditorState): Uint8Array => {
    const builder = new Builder(1024);
    const noteIdOffset = builder.createString(editorState.note_id);
    const baseKeymapOffset = builder.createString(editorState.baseKeymap)
    // const ctationOffsets = editorState.citations.map((cit) => {
    //     return citOffsetBody
    // })
    // builder.createObjectOffsetList
    EditorStateReflection.startCitationsVector(builder, editorState.citations.length)
    const citationsOffsets = editorState.citations.map((cit) => {
        const citKeyOffset = builder.createString(cit.citation_key)
        const citHtmlOffset = builder.createString(cit.html)
        return EditorCitationReflection.createEditorCitationReflection(builder, citKeyOffset, citHtmlOffset)
    })
    const citationsOffsetsVector = EditorStateReflection.createCitationsVector(builder, citationsOffsets)
    const keymapOffset = builder.createString(editorState.keymap)
    const themeOffset = builder.createString(editorState.theme)
    EditorStateReflection.startTagsVector(builder, editorState.tags.length)
    const tagsOffsets = editorState.tags.map((t) => {
        const bodyOffset = builder.createString(t.body)
        return EditorTagReflection.createEditorTagReflection(builder, bodyOffset)
    })
    const tagOffsetsVector = EditorStateReflection.createTagsVector(builder, tagsOffsets)
    EditorStateReflection.startAllCitationIdsVector(builder, editorState.allCitationIds.length)
    for (const citationId of editorState.allCitationIds) {
        builder.addOffset(builder.createString(citationId))
    }
    const citationIdOffsets = builder.endVector()
    const valueOffset = builder.createString(editorState.value)
    const parsedValueOffset = builder.createString(editorState.parsedValue)
    // -- Snippet State ---
    SnippetStateReflection.startSnippetStateReflection(builder)
    SnippetStateReflection.addIncludeEmojiSnippets(builder, editorState.snippetProps.includeEmojiSnippets)
    const snippetStateOffset = SnippetStateReflection.endSnippetStateReflection(builder)

    // -- Begin actually assembling state --
    EditorStateReflection.startEditorStateReflection(builder)
    EditorStateReflection.addNoteId(builder, noteIdOffset)
    EditorStateReflection.addBaseKeymap(builder, baseKeymapOffset)
    EditorStateReflection.addCitations(builder, citationsOffsetsVector)
    EditorStateReflection.addKeymap(builder, keymapOffset)
    EditorStateReflection.addTheme(builder, themeOffset)
    EditorStateReflection.addTags(builder, tagOffsetsVector)
    EditorStateReflection.addAllCitationIds(builder, citationIdOffsets)
    EditorStateReflection.addValue(builder, valueOffset)
    EditorStateReflection.addParsedValue(builder, parsedValueOffset)
    EditorStateReflection.addHaveSetInitialValue(builder, editorState.haveSetInitialValue)
    EditorStateReflection.addEditorView(builder, editorViewReflect(editorState.editorView))
    EditorStateReflection.addSnippetProps(builder, snippetStateOffset)
    EditorStateReflection.addLockEditorScrollToPreview(builder, editorState.lockEditorScrollToPreview)
    EditorStateReflection.addSaveMethod(builder, saveMethodReflect(editorState.saveMethod))
    EditorStateReflection.addAutoSaveTimeout(builder, editorState.autoSaveTimeout)
    const editorStateOffset = EditorStateReflection.endEditorStateReflection(builder)
    builder.finish(editorStateOffset)
    return builder.asUint8Array()
};
