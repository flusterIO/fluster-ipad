//
//  File.swift
//  FlusterWebviewClients
//
//  Created by Andrew on 3/4/26.
//

import FlatBuffers
import FlusterData
import Foundation

/// Takes common Swift language objects, creates an `AnyCrossLanguageEditorAction` and sends it to Typescript.
extension EditorState {
  public static func encodeData(data: Encodable) -> String? {
    let encoder = JSONEncoder()
    #if DEBUG
      encoder.outputFormatting = .prettyPrinted
    #endif

    do {
      let jsonData = try encoder.encode(data)
      let jsonString = String(data: jsonData, encoding: .utf8)
      return jsonString
    } catch {
      print("Error encoding user: \(error.localizedDescription)")
    }
    return nil
  }

  public static func setInitialEditorState(
    payload: EditorInitialStatePayload, eval: @escaping EvalJavascriptFunc
  ) async throws {
    let action = SetEditorInitialStateEditorAction(type: .setInitialEditorState, payload: payload)
    if let parsedData = EditorState.encodeData(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
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
      window.dispatchEvent(new CustomEvent("\(SplitviewEditorWebviewEvents.editorStateParsedContentUpdate.rawValue)", {
          detail: \(builder.sizedByteArray)
      }))
      """)
  }

  public static func setEditorSaveMethod(
    saveMethod: EditorSaveMethod, eval: @escaping EvalJavascriptFunc
  ) async throws {
    let action = SetEditorSaveMethodEditorAction(type: .setEditorSaveMethod, payload: saveMethod)
    if let parsedData = EditorState.encodeData(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }
}
