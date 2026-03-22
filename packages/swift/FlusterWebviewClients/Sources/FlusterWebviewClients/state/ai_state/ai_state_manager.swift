//
//  File.swift
//  FlusterWebviewClients
//
//  Created by Andrew on 3/22/26.
//

import FlusterData
import Foundation

@MainActor
extension AiState {
  public static func setAiThinking(thinking: Bool, eval: @escaping EvalJavascriptFunc) async throws
  {
    let action = SetAiThinkingAction(
      type: AiAction.setAiThinking, payload: SetAiThinkingPayload(ai_thinking: thinking))
    if let parsedData = await EditorState.encodeAction(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }

  public static func setFoundationModelAvailability(
    accessStatus: FoundationModelAccessStatus, eval: @escaping EvalJavascriptFunc
  ) async throws {
    let action = SetFoundationModelAvailabilityAction(
      type: AiAction.setFoundationModelAvailability,
      payload: SetFoundationModelAvailabilityPayload(foundation_model_availability: accessStatus))
    if let parsedData = await EditorState.encodeAction(data: action) {
      try await MdxEditorClient.sendEditorStateUpdate(data: parsedData, evalulateJavaScript: eval)
    }
  }
}
