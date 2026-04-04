//
//  File.swift
//  FlusterWebviewClients
//
//  Created by Andrew on 3/4/26.
//

import FlatBuffers
import FlusterData
import SwiftUI

/// Takes common Swift language objects, creates an `AnyCrossLanguageEditorAction` and sends it to Typescript.
@MainActor
extension EditorState {
  /// A simple utility function for encoding data.
  public static func encodeAction<T: Encodable>(data: T) -> String? {
    let encoder = JSONEncoder()

    // Ensure dates are ISO8601 for TypeScript compatibility
    encoder.dateEncodingStrategy = .iso8601

    do {
      let jsonData = try encoder.encode(data)
      return String(data: jsonData, encoding: .utf8)
    } catch {
      // In a library, you might want to 'throw' here
      // instead of returning nil to catch logic errors early.
      assertionFailure("Encoding failed for \(T.self): \(error)")
      return nil
    }
  }

  public static func setInitialEditorState(
    editorPayload: EditorInitialStatePayload, containerPayload: WebviewContainerSharedInitialState,
    mathPayload: MathState,
    aiPayload: AiInitialStatePayload,
    eval: @escaping EvalJavascriptFunc
  ) async throws {
    let action = SetEditorInitialStateEditorAction(
      type: .setInitialEditorState,
      payload: EditorBasedWebviewInitialState(
        container: containerPayload, editor: editorPayload, math: mathPayload, ai: aiPayload)
    )
    if let parsedAction = EditorState.encodeAction(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedAction, evalulateJavaScript: eval)
    }
  }

  public static func setParsedMdxContent(
    parsedMdxContent: String, citations: [EditorCitation], eval: @escaping EvalJavascriptFunc
  ) async throws {
    var builder = FlatBufferBuilder(initialSize: 1024)
    let parsedContentOffset = builder.create(string: parsedMdxContent)
    let citationsOffsets = citations.map { cit in
      let citationOffset = cit.serializeToFlatBufferOffset(&builder)
      return citationOffset
    }
    let citationsVectorOffset = builder.createVector(ofOffsets: citationsOffsets)
    let data =
      MdxSerialization_OnParsedContentChangeEventBuffer.createOnParsedContentChangeEventBuffer(
        &builder, parsedContentOffset: parsedContentOffset,
        citationsVectorOffset: citationsVectorOffset)
    builder.finish(offset: data)
    try await eval(
      """
      window.handleSwiftBufferAction("\(EditorStateActions.setParsedEditorContent.rawValue)", \(builder.sizedByteArray))
      """)
  }

  public static func setEditorSaveMethod(
    saveMethod: EditorSaveMethod, eval: @escaping EvalJavascriptFunc
  ) async throws {
    let action = SetEditorSaveMethodEditorAction(type: .setEditorSaveMethod, payload: saveMethod)
    if let parsedData = EditorState.encodeAction(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }

  public static func setDarkMode(colorScheme: ColorScheme, eval: @escaping EvalJavascriptFunc)
    async throws
  {
    let action = SetDarkModeAction(
      type: WebviewContainerActions.setDarkMode,
      payload: SetDarkModePayload(dark_mode: colorScheme == .dark))
    if let parsedData = EditorState.encodeAction(data: action) {
      print("Parsed Data: \(parsedData)")
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }

  public static func setEditorContent(content: String, eval: @escaping EvalJavascriptFunc)
    async throws
  {
    let action = SetEditorContentAction(
      type: EditorStateActions.setEditorValue, payload: SetEditorContentPayload(value: content))
    if let parsedData = EditorState.encodeAction(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }

  public static func setEditorBibContent(content: String, eval: @escaping EvalJavascriptFunc)
    async throws
  {
    let action = SetEditingBibEntryAction(
      type: EditorStateActions.setEditingBibEntry,
      payload: SetEditingBibEntryPayload(content: content))
    if let parsedData = EditorState.encodeAction(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }

  public static func setEditorKeymap(keymap: CodeEditorKeymap, eval: @escaping EvalJavascriptFunc)
    async throws
  {
    let action = SetEditorKeymapAction(
      type: EditorStateActions.setEditorKeymap, payload: SetEditorKeymapPayload(keymap: keymap))
    if let parsedData = EditorState.encodeAction(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }

  public static func setSnippetProps(
    payload: SetSnippetPropsPayload, eval: @escaping EvalJavascriptFunc
  )
    async throws
  {
    let action = SetSnippetPropsAction(
      type: EditorStateActions.setSnippetProps, payload: payload)
    if let parsedData = EditorState.encodeAction(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }
  public static func setEditorThemeDark(
    payload: SetEditorThemeDarkPayload, eval: @escaping EvalJavascriptFunc
  )
    async throws
  {
    let action = SetEditorThemeDarkAction(
      type: EditorStateActions.setEditorThemeDark, payload: payload)
    if let parsedData = EditorState.encodeAction(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }

  public static func setEditorThemeLight(
    payload: SetEditorThemeLightPayload, eval: @escaping EvalJavascriptFunc
  )
    async throws
  {
    let action = SetEditorThemeLightAction(
      type: EditorStateActions.setEditorThemeLight, payload: payload)
    if let parsedData = EditorState.encodeAction(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }

  public static func setLockEditorScrollToPreview(
    payload: SetLockEditorScrollToPreviewPayload, eval: @escaping EvalJavascriptFunc
  )
    async throws
  {
    let action = SetLockEditorScrollToPreviewAction(
      type: EditorStateActions.setLockEditorScrollToPreview, payload: payload)
    if let parsedData = EditorState.encodeAction(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }

  public static func setNoteDetails(payload: NoteDetailState, eval: @escaping EvalJavascriptFunc)
    async throws
  {
    let action = SetNoteDetailsAction(
      type: NoteDetailActions.setNoteDetails, payload: payload)
    if let parsedData = EditorState.encodeAction(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }
    

    
}
